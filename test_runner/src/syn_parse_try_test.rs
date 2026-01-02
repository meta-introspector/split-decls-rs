use syn::parse_file;
use std::fs;

fn main() {
    let file_path = "src/manual_tests/try_block_test.rs";
    
    println!("🧪 Testing syn parsing of try block file:");
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    
    match parse_file(&content) {
        Ok(_) => println!("✅ Syn parses the try block file successfully!"),
        Err(e) => println!("❌ Syn failed to parse: {}", e),
    }
}
