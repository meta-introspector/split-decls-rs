// Original code (before macro patch)
use std::process::Command;

fn main() {
    let output = Command::new("ls").arg("-la").output().unwrap();
    println!("Files: {}", String::from_utf8_lossy(&output.stdout));
    
    Command::new("echo").arg("hello").status().unwrap();
}
