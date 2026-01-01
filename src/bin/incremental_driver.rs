use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;
use std::io::Read;
use anyhow::Result;
use syn;
use toml;
use split_decls_genesis::preprocessing::preprocess_content;

/// Count string frequency in content
fn count_strings(content: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in content.split_whitespace() {
        let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
        if clean_word.len() > 2 {
            *counts.entry(clean_word).or_insert(0) += 1;
        }
    }
    counts
}

/// Count strings in binary using strings command
fn count_binary_strings(binary_path: &str) -> HashMap<String, usize> {
    match Command::new("strings").arg(binary_path).output() {
        Ok(output) if output.status.success() => {
            let content = String::from_utf8_lossy(&output.stdout);
            count_strings(&content)
        }
        _ => HashMap::new(),
    }
}

/// Print histogram
fn print_histogram(title: &str, counts: &HashMap<String, usize>, top_n: usize) {
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n📊 {} - Top {}:", title, top_n);
    for (i, (word, count)) in sorted.iter().take(top_n).enumerate() {
        println!("  {}: {} ({})", i + 1, word, count);
    }
}

/// Incremental compilation driver - adds files one by one until errors occur
pub struct IncrementalDriver {
    base_lib: String,
    files: Vec<String>,
    current_content: String,
}

impl IncrementalDriver {
    pub fn new() -> Result<Self> {
        // Create minimal base library without problematic modules
        let base_lib = r#"
#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

extern crate tracing;
extern crate synstructure;
extern crate proc_macro;

// Include wrap_types for basic infrastructure
include!("wrap_types.rs");
"#.to_string();
        
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
        
        let mut successful_files = 0;
        
        for (i, file) in self.files.iter().enumerate() {
            println!("\n📁 Adding file {}/{}: {}", i + 1, self.files.len(), file);
            
            // Read and preprocess the file content
            let raw_content = fs::read_to_string(format!("src/{}", file))?;
            let processed_content = preprocess_content(&raw_content);
            
            // Count strings in source
            let source_counts = count_strings(&processed_content);
            
            // Add include to current content
            self.current_content.push_str(&format!("\ninclude!(\"{}\");\n", file));
            
            // Write temporary lib.rs
            fs::write("src/lib_temp.rs", &self.current_content)?;
            
            // Try to compile
            let compile_result = Command::new("cargo")
                .args(&["build", "--lib"])
                .output()?;
            
            if compile_result.status.success() {
                println!("✅ Compilation successful");
                
                // Count strings in binary if it exists
                if Path::new("target/debug/libsplit_decls_genesis.rlib").exists() {
                    let binary_counts = count_binary_strings("target/debug/libsplit_decls_genesis.rlib");
                    print_histogram(&format!("STEP {} SOURCE", i + 1), &source_counts, 10);
                    print_histogram(&format!("STEP {} BINARY", i + 1), &binary_counts, 10);
                } else {
                    print_histogram(&format!("STEP {} SOURCE", i + 1), &source_counts, 10);
                }
            } else {
                println!("❌ Compilation failed at step {}", i + 1);
                let stderr = String::from_utf8_lossy(&compile_result.stderr);
                println!("Error: {}", stderr);
                // Continue instead of breaking
            }
            
            // Try to compile
            let success = self.try_compile()?;
            
            if !success {
                println!("❌ COMPILATION FAILED at file: {}", file);
                
                // Show the specific errors
                self.show_compilation_errors()?;
                
                // Create syn-based parser
                self.create_syn_parser(file)?;
                break;
            } else {
                println!("✅ File {} compiled successfully", file);
                successful_files += 1;
            }
        }
        
        // Final summary
        println!("\n📊 FINAL RESULTS: Successfully compiled {} out of {} files", successful_files, self.files.len());
        
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
    
    pub fn create_syn_parser(&self, failed_file: &str) -> Result<()> {
        println!("\n🔬 Creating syn-based parser for: {}", failed_file);
        
        let content = fs::read_to_string(format!("src/{}", failed_file))?;
        
        // Use syn to parse the file
        match syn::parse_file(&content) {
            Ok(ast) => {
                let mut accumulated_decls = std::collections::HashMap::new();
                
                for item in &ast.items {
                    match item {
                        syn::Item::Use(use_item) => {
                            let (module, types) = self.extract_use_info(&use_item.tree);
                            if !module.is_empty() && !types.is_empty() {
                                println!("📦 Found use: {} -> {:?}", module, types);
                                
                                let mut module_decls = Vec::new();
                                for type_name in types {
                                    module_decls.push(format!("    #[derive(Copy, Clone, Debug, PartialEq)]\n    pub struct {};", type_name));
                                }
                                accumulated_decls.insert(module, module_decls);
                            }
                        }
                        syn::Item::Mod(mod_item) => {
                            let module_name = mod_item.ident.to_string();
                            println!("📁 Found mod: {}", module_name);
                        }
                        _ => {}
                    }
                }
                
                // Generate stubs
                let mut complete_stubs = String::new();
                for (module_name, decls) in accumulated_decls {
                    complete_stubs.push_str(&format!("pub mod {} {{\n", module_name));
                    for decl in decls {
                        complete_stubs.push_str(&decl);
                        complete_stubs.push('\n');
                    }
                    complete_stubs.push_str("}\n\n");
                }
                
                fs::write("syn_generated_stubs.rs", &complete_stubs)?;
                println!("✅ Generated syn_generated_stubs.rs");
            }
            Err(e) => {
                println!("❌ Syn parse error: {}", e);
            }
        }
        
        Ok(())
    }
    
    fn extract_use_info(&self, tree: &syn::UseTree) -> (String, Vec<String>) {
        match tree {
            syn::UseTree::Path(path) => {
                let module = path.ident.to_string();
                let (_, types) = self.extract_use_info(&path.tree);
                (module, types)
            }
            syn::UseTree::Group(group) => {
                let mut types = Vec::new();
                for item in &group.items {
                    let (_, mut item_types) = self.extract_use_info(item);
                    types.append(&mut item_types);
                }
                (String::new(), types)
            }
            syn::UseTree::Name(name) => {
                (String::new(), vec![name.ident.to_string()])
            }
            _ => (String::new(), Vec::new()),
        }
    }
}

fn main() -> Result<()> {
    let mut driver = IncrementalDriver::new()?;
    driver.run_incremental_compilation()?;
    Ok(())
}
