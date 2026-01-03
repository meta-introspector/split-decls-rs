use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use std::process::Command;
use syn::parse_file;

pub struct CrateProcessor {
    crates: HashMap<String, CrateInfo>,
    dependency_graph: HashMap<String, Vec<String>>,
    build_order: Vec<String>,
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
            build_order: Vec::new(),
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
            }
        }
        
        // Extract real dependencies from source code
        self.extract_dependencies_from_source()?;
        
        // Calculate build order
        self.build_order = self.topological_sort()?;
        
        println!("📦 Discovered {} crates", self.crates.len());
        println!("🔗 Extracted {} dependency relationships", 
            self.dependency_graph.values().map(|v| v.len()).sum::<usize>());
        
        // Print the discovered plan
        self.print_build_plan()?;
        
        Ok(())
    }
    
    pub fn get_build_order(&self) -> &Vec<String> {
        &self.build_order
    }
    
    pub fn print_build_plan(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🎯 DISCOVERED BUILD PLAN");
        println!("{}", "═".repeat(80));
        
        // Group by levels for cleaner output
        let mut levels: HashMap<usize, Vec<String>> = HashMap::new();
        let mut level_map: HashMap<String, usize> = HashMap::new();
        
        // Calculate levels using topological order
        for (level, crate_name) in self.build_order.iter().enumerate() {
            level_map.insert(crate_name.clone(), level);
            levels.entry(level).or_insert_with(Vec::new).push(crate_name.clone());
        }
        
        println!("\n📊 BUILD ORDER BY LEVELS:");
        for level in 0..levels.len() {
            if let Some(crates_at_level) = levels.get(&level) {
                println!("Level {:2}: {}", level, crates_at_level.join(", "));
            }
        }
        
        println!("\n🔗 DEPENDENCY DETAILS:");
        println!("{:<30} {:<8} {}", "Crate", "Level", "Dependencies");
        println!("{}", "─".repeat(80));
        
        for crate_name in &self.build_order {
            let level = level_map.get(crate_name).unwrap_or(&0);
            let empty_deps = Vec::new();
            let deps = self.dependency_graph.get(crate_name).unwrap_or(&empty_deps);
            let dep_str = if deps.is_empty() {
                "none".to_string()
            } else {
                deps.join(", ")
            };
            
            println!("{:<30} {:<8} {}", crate_name, level, dep_str);
        }
        
        println!("\n📈 PLAN SUMMARY:");
        println!("Total crates: {}", self.crates.len());
        println!("Total levels: {}", levels.len());
        println!("Total dependencies: {}", 
            self.dependency_graph.values().map(|v| v.len()).sum::<usize>());
        println!("Max dependencies per crate: {}", 
            self.dependency_graph.values().map(|v| v.len()).max().unwrap_or(0));
        
        Ok(())
    }

    fn extract_dependencies_from_source(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize empty dependency lists
        for crate_name in self.crates.keys() {
            self.dependency_graph.insert(crate_name.clone(), Vec::new());
        }
        
        // Scan source files for dependencies
        for (crate_name, crate_info) in &self.crates {
            let mut deps = HashSet::new();
            
            // Check lib.rs or main file for extern crate and use statements
            let lib_file = Path::new(&crate_info.path).join("src/lib.rs");
            if lib_file.exists() {
                if let Ok(content) = fs::read_to_string(&lib_file) {
                    deps.extend(self.extract_deps_from_content(&content));
                }
            }
            
            // Also check a few other files for additional dependencies
            for file_path in crate_info.files.iter().take(5) {
                if let Ok(content) = fs::read_to_string(file_path) {
                    deps.extend(self.extract_deps_from_content(&content));
                }
            }
            
            // Apply hints for known complex dependencies
            deps.extend(self.get_dependency_hints(crate_name));
            
            // Filter to only rustc crates we know about
            let filtered_deps: Vec<String> = deps.into_iter()
                .filter(|dep| self.crates.contains_key(dep))
                .collect();
            
            self.dependency_graph.insert(crate_name.clone(), filtered_deps);
        }
        
        Ok(())
    }
    
    fn extract_deps_from_content(&self, content: &str) -> HashSet<String> {
        let mut deps = HashSet::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Look for extern crate statements
            if line.starts_with("extern crate ") {
                if let Some(crate_name) = self.extract_crate_name_from_extern(line) {
                    if crate_name.starts_with("rustc_") {
                        deps.insert(crate_name);
                    }
                }
            }
            
            // Look for use statements with rustc crates
            if line.starts_with("use rustc_") {
                if let Some(crate_name) = self.extract_crate_name_from_use(line) {
                    deps.insert(crate_name);
                }
            }
        }
        
        deps
    }
    
    fn extract_crate_name_from_extern(&self, line: &str) -> Option<String> {
        // extern crate rustc_middle;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let name = parts[2].trim_end_matches(';');
            Some(name.to_string())
        } else {
            None
        }
    }
    
    fn extract_crate_name_from_use(&self, line: &str) -> Option<String> {
        // use rustc_middle::ty::TyCtxt;
        if let Some(start) = line.find("rustc_") {
            let rest = &line[start..];
            if let Some(end) = rest.find("::") {
                Some(rest[..end].to_string())
            } else if let Some(end) = rest.find(";") {
                Some(rest[..end].to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn get_dependency_hints(&self, crate_name: &str) -> Vec<String> {
        // Provide hints for complex dependencies that are hard to detect from source
        match crate_name {
            "rustc_driver_impl" => vec![
                "rustc_interface".to_string(),
                "rustc_session".to_string(),
                "rustc_middle".to_string(),
                "rustc_errors".to_string(),
            ],
            "rustc_interface" => vec![
                "rustc_middle".to_string(),
                "rustc_codegen_ssa".to_string(),
                "rustc_session".to_string(),
            ],
            "rustc_middle" => vec![
                "rustc_span".to_string(),
                "rustc_data_structures".to_string(),
                "rustc_hir".to_string(),
            ],
            "rustc_hir_analysis" => vec![
                "rustc_middle".to_string(),
                "rustc_hir".to_string(),
                "rustc_infer".to_string(),
            ],
            _ => vec![],
        }
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
        // Print the plan first
        println!("\n🎯 PLAN FOR {} CRATES", self.build_order.len());
        println!("Start with: {}", self.build_order.iter().take(5).cloned().collect::<Vec<_>>().join(", "));
        if self.build_order.len() > 5 {
            println!("... and {} more", self.build_order.len() - 5);
        }
        println!("{}", "═".repeat(60));
        
        // First pass: Create all Cargo.toml files
        println!("\n🏗️  Creating Cargo.toml files for all crates...");
        for crate_name in &self.build_order {
            let crate_info = &self.crates[crate_name];
            self.create_cargo_toml(crate_info)?;
        }
        println!("✅ All Cargo.toml files created");
        
        // Second pass: Process each crate
        for (i, crate_name) in self.build_order.iter().enumerate() {
            println!("\n📦 Starting on: {} [{}/{}]", crate_name, i + 1, self.build_order.len());
            
            let crate_info = &self.crates[crate_name];
            let empty_deps = Vec::new();
            let deps = self.dependency_graph.get(crate_name).unwrap_or(&empty_deps);
            
            if !deps.is_empty() {
                println!("  🔗 Dependencies: {}", deps.join(", "));
            }
            
            // Step 1: Syn check all files
            if let Err(e) = self.syn_check_crate(crate_info) {
                println!("❌ FAILED in crate {}: {}", crate_name, e);
                return Err(e);
            }
            
            // Step 2: Rustfmt all files
            if let Err(e) = self.rustfmt_crate(crate_info) {
                println!("❌ FAILED in crate {}: {}", crate_name, e);
                return Err(e);
            }
            
            // Step 3: Try to build (Cargo.toml already exists)
            if let Err(e) = self.check_crate_build(crate_info) {
                println!("❌ FAILED in crate {}: {}", crate_name, e);
                return Err(e);
            }
            
            println!("✅ Processed: {}", crate_name);
        }
        
        println!("\n🎉 ALL {} CRATES PROCESSED SUCCESSFULLY!", self.build_order.len());
        Ok(())
    }
    
    fn create_cargo_toml(&self, crate_info: &CrateInfo) -> Result<(), Box<dyn std::error::Error>> {
        let empty_deps = Vec::new();
        let deps = self.dependency_graph.get(&crate_info.name).unwrap_or(&empty_deps);
        
        let mut cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"

[dependencies]
"#,
            crate_info.name
        );
        
        // Add real dependencies from cargo tree
        for dep in deps {
            if self.crates.contains_key(dep) {
                cargo_toml.push_str(&format!("{} = {{ path = \"../{}\", version = \"0.1.0\" }}\n", dep, dep));
            }
        }
        
        let cargo_path = Path::new(&crate_info.path).join("Cargo.toml");
        fs::write(&cargo_path, cargo_toml)?;
        Ok(())
    }
    
    fn check_crate_build(&self, crate_info: &CrateInfo) -> Result<(), Box<dyn std::error::Error>> {
        println!("  🔨 Checking build...");
        
        let output = Command::new("cargo")
            .args(&["check", "--lib"])
            .current_dir(&crate_info.path)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("❌ Cargo build error in {}: {}", crate_info.name, stderr).into());
        }
        
        println!("  ✅ Build check passed");
        Ok(())
    }

    fn syn_check_crate(&self, crate_info: &CrateInfo) -> Result<(), Box<dyn std::error::Error>> {
        println!("  🔍 Syn checking {} files...", crate_info.files.len());
        
        for file_path in &crate_info.files {
            let content = fs::read_to_string(file_path)?;
            
            if let Err(e) = parse_file(&content) {
                println!("❌ Syn parse error in {}: {}", file_path, e);
                
                // AUTO-BISECT: Find the original file and run bisection
                if let Err(bisect_error) = self.auto_bisect_file(file_path, &e.to_string()) {
                    println!("❌ Bisection failed: {}", bisect_error);
                }
                
                return Err(format!("❌ Syn parse error in {}: {}", file_path, e).into());
            }
        }
        
        println!("  ✅ Syn check passed");
        Ok(())
    }
    
    fn auto_bisect_file(&self, processed_file_path: &str, error: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🔍 AUTO-BISECTING TRANSFORMATIONS...");
        
        // Try to find the original source file
        let original_path = processed_file_path
            .replace("./output/processed/", "../rust/")
            .replace("/processed/", "/");
            
        if !Path::new(&original_path).exists() {
            return Err("Original source file not found for bisection".into());
        }
        
        println!("🔄 Found original: {}", original_path);
        let original_content = fs::read_to_string(&original_path)?;
        
        // Test original file first
        println!("🔄 Testing original file...");
        if let Err(e) = parse_file(&original_content) {
            return Err(format!("❌ Original file already has syn errors: {}", e).into());
        }
        
        println!("✅ Original file parses correctly");
        
        // Test each transformation individually
        use crate::transform_bootstrap::add_bootstrap_features;
        use crate::transform_jobserver::fix_jobserver_imports;
        use crate::transform_platform::remove_platform_specific;
        use crate::transform_tests::remove_test_code;
        use crate::transform_unused::remove_unused_code;
        use crate::transform_diagnostics::remove_diagnostic_attributes;
        
        let transformations = [
            ("bootstrap features", add_bootstrap_features as fn(&str) -> String),
            ("jobserver imports", fix_jobserver_imports),
            ("platform specific", remove_platform_specific),
            ("test code", remove_test_code),
            ("unused code", remove_unused_code),
            ("diagnostic attributes", remove_diagnostic_attributes),
        ];
        
        for (name, transform) in &transformations {
            println!("🔄 Testing {} transformation...", name);
            let transformed = transform(&original_content);
            
            if let Err(e) = parse_file(&transformed) {
                println!("❌ {} transformation broke the file: {}", name, e);
                
                // Show the problematic transformation
                println!("\n📄 PROBLEMATIC TRANSFORMATION: {}", name);
                println!("📄 Error: {}", e);
                println!("📄 Quick fix: Review the {} transformation logic", name);
                
                return Ok(());
            }
        }
        
        println!("✅ All individual transformations work, issue may be in combination");
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
        
        // Create minimal Cargo.toml for individual crate
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
    
    pub fn generate_workspace(&self, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🏗️  Generating workspace Cargo.toml...");
        
        // Generate workspace members list
        let mut members = Vec::new();
        for crate_name in &self.build_order {
            members.push(format!("  \"processed/compiler/{}\"", crate_name));
        }
        
        let workspace_toml = format!(
            r#"[workspace]
resolver = "2"
members = [
# Generated rustc compiler crates
{}
]

[workspace.dependencies]
# Common dependencies for all rustc crates
syn = {{ version = "2.0", features = ["full", "extra-traits"] }}
quote = "1.0"
proc-macro2 = "1.0"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
indexmap = "2.0"
smallvec = {{ version = "1.0", features = ["union"] }}
"#,
            members.join(",\n")
        );
        
        let workspace_path = Path::new(output_dir).join("Cargo.toml");
        fs::write(&workspace_path, workspace_toml)?;
        
        println!("✅ Workspace generated with {} members", self.build_order.len());
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
                if let Some(degree) = in_degree.get_mut(dep) {
                    *degree += 1;
                }
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
