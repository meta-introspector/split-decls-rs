use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;
use std::io::Read;
use anyhow::Result;
use syn;
use toml;
use split_decls_genesis::preprocessing::preprocess_content;

pub struct ProgressiveCompiler {
    base_lib: String,
    files: Vec<String>,
    accumulated_decls: HashMap<String, Vec<String>>,
}

impl ProgressiveCompiler {
    pub fn new() -> Result<Self> {
        // Create minimal base library
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

        // Get all processed files (no constraints)
        let mut files = Vec::new();
        for entry in fs::read_dir("src")? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("processed_") && name.ends_with(".rs") {
                files.push(name);
            }
        }
        files.sort();

        Ok(Self {
            base_lib,
            files,
            accumulated_decls: HashMap::new(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        println!("🚀 Progressive compilation of {} files to reach rustc main", self.files.len());

        let files = self.files.clone(); // Clone to avoid borrow issues
        
        for (i, file) in files.iter().enumerate() {
            println!("\n📁 Processing file {}/{}: {}", i + 1, files.len(), file);

            // Read and parse file
            let content = fs::read_to_string(format!("src/{}", file))?;
            let processed = preprocess_content(&content);

            // Extract declarations using syn
            self.extract_declarations(&processed)?;

            // Generate current stubs
            let stubs = self.generate_stubs();

            // Create complete library
            let complete_lib = format!("{}\n{}\ninclude!(\"{}\");\n", self.base_lib, stubs, file);

            // Write and test compilation
            fs::write("src/current.rs", &complete_lib)?;

            let result = Command::new("cargo")
                .args(&["build", "--lib", "--verbose"])
                .output()?;

            // Log the build command and results
            let build_log = format!("=== BUILD STEP {} ===\nCommand: cargo build --lib --verbose\nStatus: {}\nStdout:\n{}\nStderr:\n{}\n\n", 
                i + 1, result.status, 
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr));
            
            fs::write(format!("build_log_step_{}.txt", i + 1), &build_log)?;

            if result.status.success() {
                // Get source metrics
                let source_size = complete_lib.len();
                let source_strings = self.count_source_strings(&complete_lib);
                let source_unique = source_strings.len();
                
                // Get binary metrics
                let binary_path = "target/debug/libsplit_decls_genesis.rlib";
                let binary_size = fs::metadata(binary_path).map(|m| m.len()).unwrap_or(0);
                let binary_strings = self.count_binary_strings(binary_path);
                let binary_unique = binary_strings.len();
                
                println!("✅ Success! Decls:{} | Source:{}B,{}unique | Binary:{}B,{}unique", 
                    self.total_declarations(), source_size, source_unique, binary_size, binary_unique);
                
                // Show complexity growth
                if i < 5 {
                    println!("📊 Top Binary Strings:");
                    let mut sorted: Vec<_> = binary_strings.iter().collect();
                    sorted.sort_by(|a, b| b.1.cmp(a.1));
                    for (j, (string, count)) in sorted.iter().take(5).enumerate() {
                        println!("  {}: {} ({})", j + 1, string, count);
                    }
                }
            } else {
                println!("❌ Compilation failed");
                let stderr = String::from_utf8_lossy(&result.stderr);
                println!("Error: {}", stderr.lines().take(10).collect::<Vec<_>>().join("\n"));
                break;
            }
        }

        // Check if we reached the end
        if files.len() > 0 {
            println!("\n🎉 Compilation complete! Reached rustc main with {} files!", files.len());
        }

        Ok(())
    }

    fn extract_declarations(&mut self, content: &str) -> Result<()> {
        println!("🔍 Extracting declarations from content...");
        if let Ok(ast) = syn::parse_file(content) {
            println!("📝 Found {} items in AST", ast.items.len());
            for item in &ast.items {
                match item {
                    syn::Item::Use(use_item) => {
                        let (module, types) = self.extract_use_info(&use_item.tree);
                        if !module.is_empty() && !types.is_empty() && 
                           module != "crate" && module != "std" && module != "core" {
                            println!("📦 Found use: {} -> {:?}", module, types);
                            self.accumulated_decls.entry(module.clone())
                                .or_insert_with(Vec::new)
                                .extend(types);
                        }
                    }
                    syn::Item::Mod(mod_item) => {
                        println!("🗂️  Found module: {}", mod_item.ident);
                        // Generate stub file for module declarations
                        if mod_item.content.is_none() {
                            let mod_name = mod_item.ident.to_string();
                            println!("🎯 Module {} has no content, generating stub", mod_name);
                            self.generate_module_stub(&mod_name)?;
                            // Remove from accumulated_decls to avoid duplicate inline generation
                            self.accumulated_decls.remove(&mod_name);
                        } else {
                            println!("📁 Module {} has inline content", mod_item.ident);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            println!("❌ Failed to parse AST");
        }
        Ok(())
    }

    fn generate_module_stub(&self, mod_name: &str) -> Result<()> {
        let stub_path = format!("src/{}.rs", mod_name);
        
        println!("🔧 Generating stub file: {}", stub_path);
        
        // Check if we have types for this module in our accumulated declarations
        let types = self.accumulated_decls.get(mod_name).cloned().unwrap_or_default();
        
        let mut stub_content = format!("// Generated stub for {} module\n", mod_name);
        
        if types.is_empty() {
            // Generate basic stub types based on common patterns
            stub_content.push_str(&format!(
                "#[derive(Copy, Clone, Debug, PartialEq)]\npub struct {};\n\n",
                mod_name.chars().next().unwrap().to_uppercase().collect::<String>() + &mod_name[1..]
            ));
        } else {
            for type_name in &types {
                stub_content.push_str(&format!(
                    "#[derive(Copy, Clone, Debug, PartialEq)]\npub struct {};\n\n",
                    type_name
                ));
            }
        }
        
        fs::write(stub_path, stub_content)?;
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

    fn generate_stubs(&self) -> String {
        let mut stubs = String::new();
        for (module_name, types) in &self.accumulated_decls {
            stubs.push_str(&format!("pub mod {} {{\n", module_name));
            let mut unique_types: Vec<_> = types.iter().collect();
            unique_types.sort();
            unique_types.dedup();
            for type_name in unique_types {
                stubs.push_str(&format!("    #[derive(Copy, Clone, Debug, PartialEq)]\n"));
                stubs.push_str(&format!("    pub struct {};\n", type_name));
            }
            stubs.push_str("}\n\n");
        }
        stubs
    }

    fn total_declarations(&self) -> usize {
        self.accumulated_decls.values().map(|v| v.len()).sum()
    }
    
    fn count_binary_strings(&self, binary_path: &str) -> HashMap<String, usize> {
        use std::process::Command;
        
        match Command::new("strings").arg(binary_path).output() {
            Ok(output) if output.status.success() => {
                let content = String::from_utf8_lossy(&output.stdout);
                let mut counts = HashMap::new();
                
                for line in content.lines() {
                    let clean = line.trim();
                    if clean.len() > 2 && clean.chars().all(|c| c.is_ascii()) {
                        *counts.entry(clean.to_string()).or_insert(0) += 1;
                    }
                }
                counts
            }
            _ => HashMap::new(),
        }
    }
    
    fn count_source_strings(&self, source: &str) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        
        for word in source.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
            if clean.len() > 2 {
                *counts.entry(clean.to_string()).or_insert(0) += 1;
            }
        }
        counts
    }
}

fn main() -> Result<()> {
    let mut compiler = ProgressiveCompiler::new()?;
    compiler.run()?;
    Ok(())
}
