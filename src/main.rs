use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::path::Path;

fn main() {
    println!("Please enter a filename:");
    let mut file_path = String::new();
    stdin().read_line(&mut file_path).expect("Failed to read input");
    let path = Path::new(file_path.trim());
    let file = File::open(&path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}