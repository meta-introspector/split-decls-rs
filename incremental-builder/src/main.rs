use std::fs;
use std::process::Command;
use std::path::Path;

struct IncrementalBuilder {
    current_module: usize,
    declarations: Vec<String>,
    failed_at: Option<usize>,
}

impl IncrementalBuilder {
    fn new() -> Self {
        Self {
            current_module: 0,
            declarations: Vec::new(),
            failed_at: None,
        }
    }

    fn scan_output2(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Scanning output2 for declarations...");
        
        for entry in fs::read_dir("../output2")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let crate_path = entry.path();
                let crate_name = crate_path.file_name().unwrap().to_string_lossy();
                
                let decls_dir = crate_path.join("src/decls");
                if decls_dir.exists() {
                    self.scan_decls(&decls_dir, &crate_name)?;
                }
            }
        }
        
        println!("📊 Found {} declarations", self.declarations.len());
        Ok(())
    }

    fn scan_decls(&mut self, dir: &Path, crate_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let file_name = path.file_stem().unwrap().to_string_lossy();
                let relative_path = path.strip_prefix("../output2").unwrap_or(&path);
                let decl_entry = format!("{}::{} = \"{}\"", crate_name, file_name, relative_path.display());
                self.declarations.push(decl_entry);
            } else if path.is_dir() {
                self.scan_decls(&path, crate_name)?;
            }
        }
        Ok(())
    }

    fn build_incrementally(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all("../incremental-crates")?;
        
        println!("🚀 Starting incremental build...");
        
        for (i, decl) in self.declarations.iter().enumerate() {
            self.current_module = i + 1;
            
            println!("🔧 Building crate {} with declaration: {}", self.current_module, decl);
            
            self.create_crate(self.current_module, &self.declarations[..=i])?;
            
            if !self.test_compile_crate(self.current_module)? {
                println!("❌ Compilation failed at crate {}", self.current_module);
                self.failed_at = Some(self.current_module);
                self.save_progress()?;
                break;
            }
            
            println!("✅ Crate {} compiles successfully", self.current_module);
            
            // Save progress every 100 crates
            if self.current_module % 100 == 0 {
                self.save_progress()?;
            }
        }
        
        if self.failed_at.is_none() {
            println!("🎉 All {} crates compiled successfully!", self.current_module);
        }
        
        Ok(())
    }

    fn create_crate(&self, num: usize, decls: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let crate_dir = format!("../incremental-crates/crate-{:05}", num);
        fs::create_dir_all(&format!("{}/src", crate_dir))?;
        
        // Create Cargo.toml
        let cargo_toml = format!(
            "[package]\nname = \"crate-{:05}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
            num
        );
        fs::write(format!("{}/Cargo.toml", crate_dir), cargo_toml)?;
        
        // Create lib.rs with all declarations so far
        let mut lib_content = String::from("// Incremental crate with declarations 1 to ");
        lib_content.push_str(&num.to_string());
        lib_content.push_str("\n\n");
        
        for (i, decl) in decls.iter().enumerate() {
            lib_content.push_str(&format!("// Declaration {}: {}\n", i + 1, decl));
        }
        
        lib_content.push_str("\npub fn test_function() {\n    println!(\"Crate compiles with {} declarations\");\n}\n");
        
        fs::write(format!("{}/src/lib.rs", crate_dir), lib_content)?;
        
        Ok(())
    }

    fn test_compile_crate(&self, num: usize) -> Result<bool, Box<dyn std::error::Error>> {
        let crate_dir = format!("../incremental-crates/crate-{:05}", num);
        
        let output = Command::new("cargo")
            .args(&["check"])
            .current_dir(&crate_dir)
            .output()?;
        
        Ok(output.status.success())
    }

    fn save_progress(&self) -> Result<(), Box<dyn std::error::Error>> {
        let progress = format!(
            "# Incremental Build Progress\ncurrent_module = {}\ntotal_declarations = {}\nfailed_at = {:?}\n",
            self.current_module,
            self.declarations.len(),
            self.failed_at
        );
        
        fs::write("../incremental-progress.toml", progress)?;
        println!("💾 Progress saved: {} crates processed", self.current_module);
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  Incremental Crate Builder");
    
    let mut builder = IncrementalBuilder::new();
    builder.scan_output2()?;
    builder.build_incrementally()?;
    
    Ok(())
}
