use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::Result;
use split_decls_genesis::preprocessing::preprocess_content;

/// Incremental compilation driver - adds files one by one until errors occur
pub struct IncrementalDriver {
    base_lib: String,
    files: Vec<String>,
    current_content: String,
}

impl IncrementalDriver {
    pub fn new() -> Result<Self> {
        let base_lib = fs::read_to_string("src/lib.rs")?;
        
        // Get all processed rustc files
        let mut files = Vec::new();
        for entry in fs::read_dir("src")? {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("processed_rustc_") && name.ends_with(".rs") {
                    files.push(name.to_string());
                }
            }
        }
        files.sort();
        
        Ok(Self {
            base_lib: base_lib.clone(),
            files,
            current_content: base_lib,
        })
    }
    
    pub fn run_incremental_compilation(&mut self) -> Result<()> {
        println!("🚀 Starting incremental compilation with {} files", self.files.len());
        
        for (i, file) in self.files.iter().enumerate() {
            println!("\n📁 Adding file {}/{}: {}", i + 1, self.files.len(), file);
            
            // Read and preprocess the file content
            let raw_content = fs::read_to_string(format!("src/{}", file))?;
            let _file_content = preprocess_content(&raw_content);
            
            // Add include to current content
            self.current_content.push_str(&format!("\ninclude!(\"{}\");\n", file));
            
            // Write temporary lib.rs
            fs::write("src/lib_temp.rs", &self.current_content)?;
            
            // Try to compile
            let success = self.try_compile()?;
            
            if !success {
                println!("❌ COMPILATION FAILED at file: {}", file);
                println!("📊 Successfully compiled {} out of {} files", i, self.files.len());
                
                // Show the specific errors
                self.show_compilation_errors()?;
                break;
            } else {
                println!("✅ File {} compiled successfully", file);
            }
        }
        
        // Clean up
        let _ = fs::remove_file("src/lib_temp.rs");
        
        Ok(())
    }
    
    fn try_compile(&self) -> Result<bool> {
        let output = Command::new("rustc")
            .args(&[
                "--crate-type", "lib",
                "--edition", "2021",
                "src/lib_temp.rs",
                "-o", "/tmp/test_compile.rlib"
            ])
            .output()?;
        
        Ok(output.status.success())
    }
    
    fn show_compilation_errors(&self) -> Result<()> {
        let output = Command::new("rustc")
            .args(&[
                "--crate-type", "lib", 
                "--edition", "2021",
                "src/lib_temp.rs",
                "-o", "/tmp/test_compile.rlib"
            ])
            .output()?;
        
        println!("\n🔍 COMPILATION ERRORS:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut driver = IncrementalDriver::new()?;
    driver.run_incremental_compilation()?;
    Ok(())
}
