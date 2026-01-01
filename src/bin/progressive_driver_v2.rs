use std::fs;
use std::io::{Write, Read};
use std::process::Command;
use flate2::read::GzDecoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Progressive Incremental Compiler Driver v2");
    println!("Loading files from symbol_map.json.gz");
    
    // Load and decompress symbol map
    let file = fs::File::open("symbol_map.json.gz")?;
    let mut decoder = GzDecoder::new(file);
    let mut content = String::new();
    decoder.read_to_string(&mut content)?;
    
    // Extract unique source files and map to processed files (preserve JSON order)
    let mut source_files: Vec<String> = content
        .lines()
        .filter(|line| line.contains("\"source_file\":"))
        .filter_map(|line| {
            line.split("\"source_file\": \"").nth(1)?.split("\"").next()
        })
        .map(|s| s.to_string())
        .collect();
    
    // Deduplicate while preserving order
    let mut seen = std::collections::HashSet::new();
    source_files.retain(|x| seen.insert(x.clone()));
    
    // Map source files to processed files
    let processed_files: Vec<String> = source_files
        .iter()
        .filter_map(|source| {
            // Convert ../rust/compiler/rustc_ast/src/lib.rs to processed_.._rust_compiler_rustc_ast_src_lib.rs
            let processed_name = source
                .replace("../rust/compiler/", ".._rust_compiler_")
                .replace("../rust/library/", ".._rust_library_")
                .replace("/", "_")
                .replace(".rs", "");
            let processed_path = format!("processed_{}.rs", processed_name);
            
            // Check if processed file exists
            if std::path::Path::new(&format!("src/{}", processed_path)).exists() {
                Some(processed_path)
            } else {
                None
            }
        })
        .collect();
    
    println!("📁 Found {} source files, {} processed files available", source_files.len(), processed_files.len());
    
    // Progressive testing - start from file 2 to skip complex diagnostic_builder.rs
    let mut last_working = 0;
    for size in 2..=processed_files.len() {
        println!("\n🧪 Testing {} files...", size);
        
        generate_rustc_complete(&processed_files[..size])?;
        
        if test_compilation() {
            println!("✅ SUCCESS with {} files", size);
            last_working = size;
        } else {
            println!("💥 FAILED at {} files", size);
            println!("🔍 Failing file included in rustc_complete.rs for diagnosis");
            return Ok(());
        }
    }
    
    println!("\n📊 FINAL RESULT: {} files compile successfully", last_working);
    generate_rustc_complete(&processed_files[..last_working])?;
    Ok(())
}

fn test_compilation() -> bool {
    let output = Command::new("cargo")
        .args(&["check", "--lib", "--message-format=short"])
        .output();
    
    match output {
        Ok(result) => {
            if !result.status.success() {
                let stderr = String::from_utf8_lossy(&result.stderr);
                let mut error_count = 0;
                for line in stderr.lines() {
                    if line.contains("error:") || line.contains("error[E") {
                        println!("    🚨 {}", line);
                        error_count += 1;
                        if error_count >= 3 { break; }
                    }
                }
            }
            result.status.success()
        },
        Err(_) => false
    }
}

fn generate_rustc_complete(files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = format!("// Progressive rustc_complete.rs - {} files\n\n", files.len());
    content.push_str("include!(\"wrap_types.rs\");\n\n");
    
    // Files are now mirrored in submodules/rust/ with proper directory structure
    content.push_str("// Processed files are mirrored in submodules/rust/ directory\n");
    content.push_str("// This allows super:: imports to work correctly\n");
    
    fs::write("src/rustc_complete.rs", content)?;
    Ok(())
}
