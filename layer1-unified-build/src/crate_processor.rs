use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use std::process::Command;
use syn::parse_file;

pub struct CrateProcessor {
    crates: HashMap<String, CrateInfo>,
    dependency_graph: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
struct CrateInfo {
    name: String,
    path: String,
    files: Vec<String>,
}

impl CrateProcessor {
    pub fn new() -> Self {
        Self {
            crates: HashMap::new(),
            dependency_graph: HashMap::new(),
        }
    }

    pub fn discover_crates(&mut self, processed_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let compiler_dir = Path::new(processed_dir).join("compiler");
        
        for entry in fs::read_dir(&compiler_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let crate_name = entry.file_name().to_string_lossy().to_string();
                let crate_path = entry.path().to_string_lossy().to_string();
                
                // Find all .rs files in this crate
                let mut files = Vec::new();
                self.collect_rs_files(&entry.path(), &mut files)?;
                
                self.crates.insert(crate_name.clone(), CrateInfo {
                    name: crate_name.clone(),
                    path: crate_path,
                    files,
                });
                
                self.dependency_graph.insert(crate_name, Vec::new());
            }
        }
        
        println!("📦 Discovered {} crates", self.crates.len());
        Ok(())
    }

    fn collect_rs_files(&self, dir: &Path, files: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.collect_rs_files(&path, files)?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                files.push(path.to_string_lossy().to_string());
            }
        }
        Ok(())
    }

    pub fn process_in_topological_order(&self) -> Result<(), Box<dyn std::error::Error>> {
        let order = self.topological_sort()?;
        
        println!("🔄 Processing {} crates in topological order", order.len());
        
        for (i, crate_name) in order.iter().enumerate() {
            println!("\n📦 [{}/{}] Processing crate: {}", i + 1, order.len(), crate_name);
            
            let crate_info = &self.crates[crate_name];
            
            // Step 1: Syn check all files
            self.syn_check_crate(crate_info)?;
            
            // Step 2: Rustfmt all files
            self.rustfmt_crate(crate_info)?;
            
            // Step 3: Create minimal Cargo.toml and try to build
            self.build_crate(crate_info)?;
            
            println!("✅ Crate {} processed successfully", crate_name);
        }
        
        Ok(())
    }

    fn syn_check_crate(&self, crate_info: &CrateInfo) -> Result<(), Box<dyn std::error::Error>> {
        println!("  🔍 Syn checking {} files...", crate_info.files.len());
        
        for file_path in &crate_info.files {
            let content = fs::read_to_string(file_path)?;
            
            if let Err(e) = parse_file(&content) {
                return Err(format!("❌ Syn parse error in {}: {}", file_path, e).into());
            }
        }
        
        println!("  ✅ Syn check passed");
        Ok(())
    }

    fn rustfmt_crate(&self, crate_info: &CrateInfo) -> Result<(), Box<dyn std::error::Error>> {
        println!("  🎨 Formatting {} files...", crate_info.files.len());
        
        for file_path in &crate_info.files {
            let output = Command::new("rustfmt")
                .arg("--check")
                .arg(file_path)
                .output()?;
            
            if !output.status.success() {
                // Try to format it
                let format_output = Command::new("rustfmt")
                    .arg(file_path)
                    .output()?;
                
                if !format_output.status.success() {
                    let stderr = String::from_utf8_lossy(&format_output.stderr);
                    return Err(format!("❌ Rustfmt error in {}: {}", file_path, stderr).into());
                }
            }
        }
        
        println!("  ✅ Formatting passed");
        Ok(())
    }

    fn build_crate(&self, crate_info: &CrateInfo) -> Result<(), Box<dyn std::error::Error>> {
        println!("  🔨 Building crate...");
        
        // Create minimal Cargo.toml
        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
"#,
            crate_info.name
        );
        
        let cargo_path = Path::new(&crate_info.path).join("Cargo.toml");
        fs::write(&cargo_path, cargo_toml)?;
        
        // Try to build
        let output = Command::new("cargo")
            .args(&["check", "--lib"])
            .current_dir(&crate_info.path)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("❌ Cargo build error in {}: {}", crate_info.name, stderr).into());
        }
        
        println!("  ✅ Build passed");
        Ok(())
    }

    fn topological_sort(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        // Initialize in-degrees
        for crate_name in self.crates.keys() {
            in_degree.insert(crate_name.clone(), 0);
        }
        
        // Calculate in-degrees
        for deps in self.dependency_graph.values() {
            for dep in deps {
                *in_degree.get_mut(dep).unwrap_or(&mut 0) += 1;
            }
        }
        
        // Find nodes with no incoming edges
        for (crate_name, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(crate_name.clone());
            }
        }
        
        // Process queue
        while let Some(crate_name) = queue.pop_front() {
            result.push(crate_name.clone());
            
            if let Some(deps) = self.dependency_graph.get(&crate_name) {
                for dep in deps {
                    if let Some(degree) = in_degree.get_mut(dep) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }
        
        if result.len() != self.crates.len() {
            return Err("Circular dependency detected".into());
        }
        
        Ok(result)
    }
}
