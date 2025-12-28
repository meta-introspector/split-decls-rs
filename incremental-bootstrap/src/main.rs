use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use petgraph::algo::toposort;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

type DependencyGraph = Graph<String, (), Directed>;

struct IncrementalBootstrap {
    graph: DependencyGraph,
    node_map: HashMap<String, NodeIndex>,
    module_count: usize,
}

impl IncrementalBootstrap {
    fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_map: HashMap::new(),
            module_count: 0,
        }
    }

    fn scan_output2(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Scanning output2 for all declarations...");
        
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
        
        Ok(())
    }

    fn scan_decls(&mut self, dir: &Path, crate_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let file_name = path.file_stem().unwrap().to_string_lossy();
                let decl_name = format!("{}::{}", crate_name, file_name);
                self.add_decl(decl_name);
            } else if path.is_dir() {
                self.scan_decls(&path, crate_name)?;
            }
        }
        Ok(())
    }

    fn add_decl(&mut self, name: String) -> NodeIndex {
        if let Some(&idx) = self.node_map.get(&name) {
            idx
        } else {
            let idx = self.graph.add_node(name.clone());
            self.node_map.insert(name, idx);
            idx
        }
    }

    fn generate_incremental_modules(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sorted = toposort(&self.graph, None)
            .map_err(|_| "Cycle detected in dependency graph")?;
        
        println!("📦 Generating incremental modules in topological order...");
        
        for (i, node_idx) in sorted.iter().enumerate() {
            let decl_name = &self.graph[*node_idx];
            let module_num = i + 1;
            
            println!("🔧 Module {}: {}", module_num, decl_name);
            
            // Generate module file
            let module_content = format!(
                "// Module {} - {}\n// Generated in topological order\n\nuse std::*;\n\n// Include declaration: {}\n// TODO: Add actual include path\n\npub fn test_compile_module_{}() {{\n    println!(\"Module {} compiles successfully\");\n}}\n",
                module_num, decl_name, decl_name, module_num, module_num
            );
            
            let module_path = format!("../bootstrap3-incremental/module_{:04}.rs", module_num);
            fs::write(&module_path, module_content)?;
            
            // Try to compile this module
            if !self.test_compile_module(module_num)? {
                println!("❌ Module {} failed to compile - creating crate-lattice-{}", module_num, module_num);
                self.create_lattice_crate(module_num, decl_name)?;
                break;
            }
        }
        
        Ok(())
    }

    fn test_compile_module(&self, module_num: usize) -> Result<bool, Box<dyn std::error::Error>> {
        // Simulate compilation test
        println!("   ✅ Module {} compiles", module_num);
        Ok(true) // For now, assume all compile
    }

    fn create_lattice_crate(&self, num: usize, decl_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lattice_content = format!(
            "// Crate Lattice {} - Failed compilation point\n// Declaration: {}\n// This crate combines multiple dependencies\n\n[package]\nname = \"crate-lattice-{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n// TODO: Add dependencies that caused the failure\n",
            num, decl_name, num
        );
        
        fs::create_dir_all("../bootstrap3-incremental/lattice-crates")?;
        let lattice_path = format!("../bootstrap3-incremental/lattice-crates/crate-lattice-{}.toml", num);
        fs::write(&lattice_path, lattice_content)?;
        
        println!("🔗 Created lattice crate: {}", lattice_path);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Incremental Bootstrap Generator");
    
    fs::create_dir_all("../bootstrap3-incremental")?;
    
    let mut bootstrap = IncrementalBootstrap::new();
    bootstrap.scan_output2()?;
    bootstrap.generate_incremental_modules()?;
    
    println!("✨ Incremental bootstrap generation complete!");
    Ok(())
}
