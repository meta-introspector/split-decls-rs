// Test case for parsing error: Original content doesn't parse: expected identifier or `_`
// Original file: /home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust/library/core/src/convert/mod.rs
// Error type: expected_identifier
// Sample #1 of 3

use syn::parse_file;
use split_decls_genesis::*;
use std::fs;

fn main() {
    let source_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust/library/core/src/convert/mod.rs";
    
    println!("🔧 Running build process on: {}", source_path);
    
    // Read original source
    let original = fs::read_to_string(source_path).expect("Failed to read source file");
    println!("1️⃣ Original source loaded ({} bytes)", original.len());
    
    // Test original parsing with detailed error reporting
    match parse_file(&original) {
        Ok(_) => println!("✅ Original parses fine"),
        Err(e) => {
            println!("❌ Original source broken: {}", e);
            
            // Get the span information from the error
            let span = e.span();
            let start = span.start();
            
            println!("📍 Error at line {}, column {}", start.line, start.column);
            
            // Show context around the error
            let lines: Vec<&str> = original.lines().collect();
            let error_line = start.line.saturating_sub(1); // Convert to 0-based index
            
            let context_start = error_line.saturating_sub(10);
            let context_end = (error_line + 10).min(lines.len());
            
            println!("📝 Context around error:");
            for i in context_start..context_end {
                let line_num = i + 1;
                let marker = if line_num == start.line { ">>> " } else { "    " };
                println!("{}{:4}: {}", marker, line_num, lines[i]);
                
                // Show column pointer for the error line
                if line_num == start.line {
                    let pointer = " ".repeat(8 + start.column.saturating_sub(1)) + "^";
                    println!("    {}", pointer);
                }
            }
            
            return;
        }
    }
    
    println!("✅ Test completed successfully!");
}
