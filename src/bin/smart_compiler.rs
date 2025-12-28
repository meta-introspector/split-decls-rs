use std::collections::{HashMap, VecDeque};
use std::path::Path;
use anyhow::Result;

macro_rules! includedeps {
    () => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/dependency_data.rs"));
    };
}

includedeps!();

pub struct SmartCompiler {
    dependencies: HashMap<&'static str, Vec<&'static str>>,
    build_order: Vec<&'static str>,
    compiled: HashMap<String, CompiledDecl>,
}

#[derive(Debug, Clone)]
pub struct CompiledDecl {
    name: String,
    content: String,
    dependencies: Vec<String>,
}

impl SmartCompiler {
    pub fn new() -> Self {
        let dependencies = dependency_data!();
        let build_order = Self::topological_sort(&dependencies);
        
        Self {
            dependencies,
            build_order,
            compiled: HashMap::new(),
        }
    }

    fn topological_sort(deps: &HashMap<&'static str, Vec<&'static str>>) -> Vec<&'static str> {
        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        
        // Build reverse graph and calculate in-degrees
        for (&node, dependencies) in deps {
            in_degree.entry(node).or_insert(0);
            for &dep in dependencies {
                graph.entry(dep).or_default().push(node);
                *in_degree.entry(node).or_insert(0) += 1;
            }
        }
        
        // Kahn's algorithm
        let mut queue: VecDeque<&str> = in_degree.iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(node, _)| *node)
            .collect();
        
        let mut result = Vec::new();
        
        while let Some(node) = queue.pop_front() {
            result.push(node);
            
            if let Some(neighbors) = graph.get(node) {
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
        
        result
    }

    pub fn compile_all(&mut self) -> Result<()> {
        println!("🔄 Compiling {} declarations in dependency order...", self.build_order.len());
        
        let build_order = self.build_order.clone(); // Clone to avoid borrowing issues
        for decl_name in &build_order {
            self.compile_declaration(decl_name)?;
        }
        
        println!("✅ All declarations compiled successfully");
        Ok(())
    }

    fn compile_declaration(&mut self, name: &str) -> Result<()> {
        if self.compiled.contains_key(name) {
            return Ok(());
        }

        let deps = self.dependencies.get(name).cloned().unwrap_or_default();
        let content = format!("// Declaration: {}\n// Dependencies: {:?}", name, deps);
        
        let compiled = CompiledDecl {
            name: name.to_string(),
            content,
            dependencies: deps.into_iter().map(|s| s.to_string()).collect(),
        };
        
        self.compiled.insert(name.to_string(), compiled);
        Ok(())
    }

    pub fn query(&self, name: &str) -> Option<&CompiledDecl> {
        self.compiled.get(name)
    }

    pub fn list_compiled(&self) -> Vec<&str> {
        self.compiled.keys().map(|s| s.as_str()).collect()
    }

    pub fn dependency_chain(&self, name: &str) -> Vec<String> {
        let mut chain = Vec::new();
        let mut visited = std::collections::HashSet::new();
        self.collect_deps(name, &mut chain, &mut visited);
        chain
    }

    fn collect_deps(&self, name: &str, chain: &mut Vec<String>, visited: &mut std::collections::HashSet<String>) {
        if visited.contains(name) {
            return;
        }
        visited.insert(name.to_string());
        
        if let Some(deps) = self.dependencies.get(name) {
            for &dep in deps {
                self.collect_deps(dep, chain, visited);
            }
        }
        chain.push(name.to_string());
    }
}

fn main() -> Result<()> {
    let mut compiler = SmartCompiler::new();
    
    println!("🧠 Smart Compiler initialized with {} dependencies", compiler.dependencies.len());
    println!("📋 Build order: {} items", compiler.build_order.len());
    
    compiler.compile_all()?;
    
    // Interactive mode
    println!("\n💬 Interactive mode - type declaration names to query:");
    println!("Commands: 'list' (show all), 'deps <name>' (show dependency chain), 'quit'");
    
    loop {
        print!("> ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        match input {
            "quit" => break,
            "list" => {
                let compiled = compiler.list_compiled();
                println!("📦 {} compiled declarations:", compiled.len());
                for (i, name) in compiled.iter().take(10).enumerate() {
                    println!("  {}. {}", i + 1, name);
                }
                if compiled.len() > 10 {
                    println!("  ... and {} more", compiled.len() - 10);
                }
            }
            cmd if cmd.starts_with("deps ") => {
                let name = &cmd[5..];
                let chain = compiler.dependency_chain(name);
                println!("🔗 Dependency chain for '{}': {:?}", name, chain);
            }
            name => {
                if let Some(decl) = compiler.query(name) {
                    println!("📄 Declaration '{}' found:", decl.name);
                    println!("   Dependencies: {:?}", decl.dependencies);
                    println!("   Content preview: {}", decl.content.lines().next().unwrap_or(""));
                } else {
                    println!("❌ Declaration '{}' not found", name);
                }
            }
        }
    }
    
    Ok(())
}
