use syn::parse_file;
use std::fs;

fn main() {
    let file_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_errors/src/translation.rs";
    
    let original_content = fs::read_to_string(file_path).expect("Failed to read file");
    let lines: Vec<&str> = original_content.lines().collect();
    
    let mut accumulated_content = String::new();
    let mut skip_count = 0;
    
    for (line_num, line) in lines.iter().enumerate() {
        // Skip incomplete doc comments
        if line.starts_with("///") && !line.trim_end().ends_with('.') && !line.trim_end().ends_with('!') {
            skip_count += 1;
            continue;
        }
        
        accumulated_content.push_str(line);
        accumulated_content.push('\n');
        
        match parse_file(&accumulated_content) {
            Ok(_) => {
                if line_num % 20 == 0 {
                    println!("✅ Line {}: OK", line_num + 1);
                }
            }
            Err(e) => {
                println!("❌ PARSING FAILED at line {}: {}", line_num + 1, e);
                println!("📍 Problematic line: '{}'", line);
                return;
            }
        }
    }
    
    println!("✅ Completed! Skipped {} incomplete doc comments", skip_count);
}
