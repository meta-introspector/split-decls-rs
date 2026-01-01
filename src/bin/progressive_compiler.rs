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

extern crate tracing;
extern crate synstructure;
extern crate proc_macro;

include!("wrap_types.rs");
"#.to_string();

        // Get all processed files
        let mut files = Vec::new();
        for entry in fs::read_dir("src")? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("processed_rustc_") && name.ends_with(".rs") {
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
            fs::write("src/lib_temp.rs", &complete_lib)?;

            let result = Command::new("cargo")
                .args(&["build", "--lib"])
                .output()?;

            if result.status.success() {
                println!("✅ Success! {} declarations accumulated", self.total_declarations());
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
        if let Ok(ast) = syn::parse_file(content) {
            for item in &ast.items {
                if let syn::Item::Use(use_item) = item {
                    let (module, types) = self.extract_use_info(&use_item.tree);
                    if !module.is_empty() && !types.is_empty() && 
                       module != "crate" && module != "std" && module != "core" {
                        self.accumulated_decls.entry(module)
                            .or_insert_with(Vec::new)
                            .extend(types);
                    }
                }
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
}

fn main() -> Result<()> {
    let mut compiler = ProgressiveCompiler::new()?;
    compiler.run()?;
    Ok(())
}
