use std::io::{self, BufReader, BufRead, Read, Write};
use std::fs::File;
use std::net::{TcpListener,TcpStream};

pub struct Http;
impl Http
{
    pub fn new(){
        println!("rust serve start bind 0.0.0.0:7878");
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            Self::handle_connection(stream);
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
        let mut lines = String::new();
        let mut reader = BufReader::new(&stream);
        reader.read_line(&mut lines);
        //"<h1>404 NOT FOUND</h1>"
        let response_404 = (vec![],"HTTP/1.1 404 NOT FOUND\r\n\r\n");
        if !lines.starts_with("GET /") {
            stream.write(response_404.1.as_bytes()).unwrap();
            stream.write(&response_404.0).unwrap();
        } else {
            let file_name = Self::parse(&lines);
            let content = Self::read_file(&file_name);
            let (content, response_line) = match content {
                Ok(file_content) => {
                    let mut arr:Vec<&str> = file_name.split(".").collect();
                    let mut response_line = "";
                    if arr.len() > 1 {
                        if let Some(file_type) = arr.pop() {
                            response_line = match file_type {
                                "html" => "HTTP/1.1 200 OK\r\n\r\n",
                                "css" => "HTTP/1.1 200 OK\r\n\r\n",
                                "js" => "HTTP/1.1 200 OK\r\n\r\n",
                                _ => "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\n\r\n",
                            }
                        }
                    } else {
                        response_line = "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\n\r\n";
                    }
                    (file_content,response_line)
                },
                Err(_) => response_404
            };
            stream.write(response_line.as_bytes()).unwrap();
            stream.write(&content).unwrap();
        }
        stream.flush().unwrap();
    }

    fn response_header(){
//
//     Content-Type
    }


    fn parse (request_line:&str) -> String {
        let mut vec_line = request_line.split_whitespace();
        let mut request_line = vec_line.nth(1).unwrap().to_string();
        let mut url:Vec<&str> = request_line.split("/").collect();
        url.remove(0);
        let file = url.join("/");
        dbg!(file)
    }

    ///
    fn read_file(filename: &str) -> io::Result<Vec<u8>> {
        let mut file_content = vec![];
        File::open(&filename)?.read_to_end(&mut file_content)?;
        Ok(file_content)
    }
}