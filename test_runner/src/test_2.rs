// Test case for parsing error: Original content doesn't parse: expected an expression
// Original file: /mnt/data1/nix/vendor/rust/cargo2nix//mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_errors/src/translation.rs
// Error type: expected_expression
// Sample #1 of 3

use syn::parse_file;
use lib_introspector_core::{test_parse, add_prelude, process_content, detailed_parse_error, apply_transformation_by_name};
use std::fs;

macro_rules! runbuild {
    ($source_path:expr) => {
        {
            println!("🔧 Running build process on: {}", $source_path);
            
            // Read original source
            let original = fs::read_to_string($source_path).expect("Failed to read source file");
            println!("1️⃣ Original source loaded ({} bytes)", original.len());
            
            // Test original parsing first with SUPER DETAILED reporting
            let (success, error) = test_parse(&original);
            println!("📊 Original parsing: {}", if success { "✅ SUCCESS" } else { "❌ FAILED" });
            if !success {
                println!("\n🔍 SUPER DETAILED ERROR ANALYSIS:");
                println!("{}", detailed_parse_error(&original));
                println!("{}", "=".repeat(80));
            }
            
            // Apply transformations step by step and test each
            let transformations = vec![
                "add_prelude",
                "strip_incomplete_docs",
                "fix_file_paths", 
                "fix_env_vars",
                "fix_attribute_spacing",
                "remove_crate_attrs",
            ];
            
            let mut current = original.clone();
            for transform_name in transformations {
                println!("\n🔄 Applying {}...", transform_name);
                current = apply_transformation_by_name(&current, transform_name);
                let (step_success, step_error) = test_parse(&current);
                println!("📊 After {}: {}", transform_name, if step_success { "✅ SUCCESS" } else { "❌ FAILED" });
                if !step_success {
                    println!("\n🔍 DETAILED ERROR AFTER {}:", transform_name);
                    println!("{}", detailed_parse_error(&current));
                    println!("{}", "=".repeat(80));
                }
            }
        }
    };
}

fn main() {
    runbuild!("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_errors/src/translation.rs");
}
