// Test case for parsing error: Original content doesn't parse: expected an expression
// Original file: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/need_type_info.rs
// Error type: expected_expression
// Sample #2 of 3

use syn::parse_file;
use split_decls_genesis::build_lib::*;
use std::fs;

macro_rules! runbuild {
    ($source_path:expr) => {
        {
            println!("🔧 Running build process on: {}", $source_path);
            
            // Read original source
            let original = fs::read_to_string($source_path).expect("Failed to read source file");
            println!("1️⃣ Original source loaded ({} bytes)", original.len());
            
            // Test original parsing
            match parse_file(&original) {
                Ok(_) => println!("✅ Original parses fine"),
                Err(e) => {
                    println!("❌ Original source broken: {}", e);
                    return;
                }
            }
            
            // Apply transformations step by step
            println!("\\n2️⃣ Adding prelude...");
            let step2 = add_prelude(&original);
            match parse_file(&step2) {
                Ok(_) => println!("✅ After prelude: Parse OK"),
                Err(e) => {
                    println!("❌ Prelude broke parsing: {}", e);
                    return;
                }
            }
            
            println!("\\n3️⃣ Running full process_content...");
            match process_content(&original) {
                Ok(result) => {
                    println!("✅ Full process completed ({} bytes)", result.len());
                    
                    // Test final result parsing
                    match parse_file(&result) {
                        Ok(_) => println!("✅ Final result parses fine"),
                        Err(e) => println!("❌ Final result broken: {}", e),
                    }
                }
                Err(e) => println!("❌ Process failed: {}", e),
            }
        }
    };
}

fn main() {
    runbuild!("../rust/compiler/rustc_trait_selection/src/error_reporting/infer/need_type_info.rs");
}
