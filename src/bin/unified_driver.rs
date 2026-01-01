use std::fs;
use std::process::Command;
use std::collections::HashMap;
use anyhow::Result;
use syn;

fn preprocess_content(content: &str) -> String {
    content
        .replace("//!", "//")  // Fix inner doc comments
        .replace("/*!", "/*")  // Fix inner block doc comments
        .replace("#![", "#[")  // Fix inner attributes
}

pub struct UnifiedDriver {
    base_lib: String,
    files: Vec<String>,
    accumulated_decls: HashMap<String, Vec<String>>,
}

impl UnifiedDriver {
    pub fn new() -> Result<Self> {
        let base_lib = r#"
#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

include!("wrap_types.rs");
"#.to_string();

        // Get all processed files
        let mut files = Vec::new();
        for entry in fs::read_dir("src")? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("processed_") && name.ends_with(".rs") {
                files.push(name);
            }
        }

        files.sort();
        println!("📁 Found {} processed files", files.len());

        Ok(Self {
            base_lib,
            files,
            accumulated_decls: HashMap::new(),
        })
    }

    pub fn run_progressive_analysis(&mut self) -> Result<()> {
        println!("🚀 Starting Progressive Analysis");
        
        for (i, file) in self.files.clone().iter().enumerate() {
            println!("\n🔄 Step {}/{}: Processing {}", i + 1, self.files.len(), file);
            
            // Read and preprocess file
            let content = fs::read_to_string(format!("src/{}", file))?;
            let processed = preprocess_content(&content);
            
            // Extract declarations
            self.extract_declarations(&processed)?;
            
            // Generate stubs for accumulated declarations
            let stubs = self.generate_stubs();
            
            // Create complete library
            let complete_lib = format!("{}\n{}\ninclude!(\"{}\");\n", self.base_lib, stubs, file);
            
            // Write and test compilation
            fs::write("src/current.rs", &complete_lib)?;
            
            let result = Command::new("cargo")
                .args(&["build", "--lib", "--verbose"])
                .output()?;
            
            // Log build results
            let build_log = format!("=== BUILD STEP {} ===\nFile: {}\nStatus: {}\nStderr:\n{}\n\n", 
                i + 1, file, result.status, String::from_utf8_lossy(&result.stderr));
            fs::write(format!("build_log_step_{}.txt", i + 1), &build_log)?;
            
            if result.status.success() {
                // Get metrics
                let source_size = complete_lib.len();
                let binary_path = "target/debug/libsplit_decls_genesis.rlib";
                let binary_size = fs::metadata(binary_path).map(|m| m.len()).unwrap_or(0);
                
                println!("✅ Success! Decls:{} | Source:{}B | Binary:{}B", 
                    self.total_declarations(), source_size, binary_size);
                    
                // Save successful state
                fs::write(format!("liblib_temp_{}.rlib", i + 1), 
                    fs::read(binary_path).unwrap_or_default())?;
            } else {
                println!("❌ Compilation failed at step {}", i + 1);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                // Print actual errors
                println!("🔍 ERROR DETAILS:");
                for line in stderr.lines() {
                    if line.contains("error[E") || line.contains("error:") {
                        println!("  🚨 {}", line);
                    }
                }
                
                if stderr.contains("error[E") {
                    println!("🔍 Errors found - stopping progressive analysis");
                    break;
                }
            }
        }
        
        Ok(())
    }

    fn extract_declarations(&mut self, content: &str) -> Result<()> {
        if let Ok(ast) = syn::parse_file(content) {
            for item in &ast.items {
                match item {
                    syn::Item::Use(use_item) => {
                        self.process_use_tree(&use_item.tree);
                    }
                    syn::Item::Mod(mod_item) => {
                        if mod_item.content.is_none() {
                            let mod_name = mod_item.ident.to_string();
                            self.generate_module_stub(&mod_name)?;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn process_use_tree(&mut self, tree: &syn::UseTree) {
        match tree {
            syn::UseTree::Path(path) => {
                let path_str = path.ident.to_string();
                if path_str == "crate" {
                    // This is a crate:: import, process the tree part
                    self.process_use_tree(&path.tree);
                }
            }
            syn::UseTree::Name(name) => {
                // Extract the name being imported
                let name_str = name.ident.to_string();
                if name_str.chars().next().unwrap().is_uppercase() {
                    // This looks like a type - add it to root crate
                    self.accumulated_decls.entry("crate".to_string())
                        .or_insert_with(Vec::new)
                        .push(name_str);
                }
            }
            syn::UseTree::Group(group) => {
                // Handle grouped imports like {Align, HasDataLayout, Size}
                for tree in &group.items {
                    self.process_use_tree(tree);
                }
            }
            _ => {}
        }
    }

    fn generate_module_stub(&self, mod_name: &str) -> Result<()> {
        let stub_path = format!("src/{}.rs", mod_name);
        let stub_content = format!("// Generated stub for {} module\n#[derive(Copy, Clone, Debug, PartialEq)]\npub struct {};\n", 
            mod_name, mod_name.chars().next().unwrap().to_uppercase().collect::<String>() + &mod_name[1..]);
        fs::write(stub_path, stub_content)?;
        Ok(())
    }

    fn extract_use_info(&self, tree: &syn::UseTree) -> (String, Vec<String>) {
        match tree {
            syn::UseTree::Path(path) => {
                let (mut module, types) = self.extract_use_info(&path.tree);
                if module.is_empty() {
                    module = path.ident.to_string();
                } else {
                    module = format!("{}::{}", path.ident, module);
                }
                (module, types)
            }
            syn::UseTree::Name(name) => {
                (String::new(), vec![name.ident.to_string()])
            }
            syn::UseTree::Group(group) => {
                let mut all_types = Vec::new();
                for item in &group.items {
                    let (_, types) = self.extract_use_info(item);
                    all_types.extend(types);
                }
                (String::new(), all_types)
            }
            _ => (String::new(), Vec::new()),
        }
    }

    fn generate_stubs(&self) -> String {
        let mut stubs = String::new();
        
        // Add crate-level types first
        if let Some(types) = self.accumulated_decls.get("crate") {
            for type_name in types {
                stubs.push_str(&format!("pub struct {};\n", type_name));
            }
            stubs.push_str("\n");
        }
        
        // Add module stubs
        for (module, types) in &self.accumulated_decls {
            if module != "crate" {
                stubs.push_str(&format!("pub mod {} {{\n", module));
                for type_name in types {
                    stubs.push_str(&format!("    pub struct {};\n", type_name));
                }
                stubs.push_str("}\n\n");
            }
        }
        stubs
    }

    fn total_declarations(&self) -> usize {
        self.accumulated_decls.values().map(|v| v.len()).sum()
    }
}

fn main() -> Result<()> {
    let mut driver = UnifiedDriver::new()?;
    driver.run_progressive_analysis()?;
    Ok(())
}
