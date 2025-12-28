use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone)]
struct LatticeNode {
    name: String,
    file_path: String,
    uses: HashSet<String>,
    level: usize,
}

struct RustLattice {
    nodes: HashMap<String, LatticeNode>,
    levels: Vec<Vec<String>>,
    dependencies: HashMap<String, HashSet<String>>,
}

impl RustLattice {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            levels: Vec::new(),
            dependencies: HashMap::new(),
        }
    }

    fn extract_uses_from_file(&self, file_path: &Path) -> Result<HashSet<String>> {
        let content = fs::read_to_string(file_path)?;
        let mut uses = HashSet::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("use ") && !line.contains("super::") {
                // Extract the used item
                if let Some(use_part) = line.strip_prefix("use ") {
                    if let Some(semicolon_pos) = use_part.find(';') {
                        let use_stmt = &use_part[..semicolon_pos];
                        // Extract the last identifier
                        if let Some(last_part) = use_stmt.split("::").last() {
                            let clean = last_part.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                            if !clean.is_empty() && clean.chars().next().unwrap().is_uppercase() {
                                uses.insert(clean.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        Ok(uses)
    }

    fn add_node(&mut self, name: String, file_path: String, uses: HashSet<String>) {
        let node = LatticeNode {
            name: name.clone(),
            file_path,
            uses: uses.clone(),
            level: 0, // Will be calculated later
        };
        
        self.nodes.insert(name.clone(), node);
        self.dependencies.insert(name, uses);
    }

    fn build_lattice_levels(&mut self) {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        
        // Calculate in-degrees and build reverse graph
        for (node, deps) in &self.dependencies {
            in_degree.entry(node.clone()).or_insert(0);
            for dep in deps {
                if self.nodes.contains_key(dep) {
                    graph.entry(dep.clone()).or_default().insert(node.clone());
                    *in_degree.entry(node.clone()).or_insert(0) += 1;
                }
            }
        }
        
        // Topological sort by levels
        let mut queue: VecDeque<String> = in_degree.iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(node, _)| node.clone())
            .collect();
        
        let mut current_level = 0;
        
        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level_nodes = Vec::new();
            
            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    current_level_nodes.push(node.clone());
                    
                    // Update node level
                    if let Some(node_data) = self.nodes.get_mut(&node) {
                        node_data.level = current_level;
                    }
                    
                    // Process dependents
                    if let Some(dependents) = graph.get(&node) {
                        for dependent in dependents {
                            if let Some(degree) = in_degree.get_mut(dependent) {
                                *degree -= 1;
                                if *degree == 0 {
                                    queue.push_back(dependent.clone());
                                }
                            }
                        }
                    }
                }
            }
            
            if !current_level_nodes.is_empty() {
                self.levels.push(current_level_nodes);
                current_level += 1;
            }
        }
    }

    fn print_lattice(&self) {
        println!("🔗 Rust Dependency Lattice");
        println!("==========================");
        
        for (level, nodes) in self.levels.iter().enumerate() {
            println!("📊 Level {}: {} nodes", level, nodes.len());
            for (i, node) in nodes.iter().take(5).enumerate() {
                if let Some(node_data) = self.nodes.get(node) {
                    println!("  {}. {} (uses: {})", i + 1, node, node_data.uses.len());
                }
            }
            if nodes.len() > 5 {
                println!("  ... and {} more", nodes.len() - 5);
            }
            println!();
        }
        
        println!("📈 Lattice Summary:");
        println!("  • {} total levels", self.levels.len());
        println!("  • {} total nodes", self.nodes.len());
        println!("  • {} dependency relationships", 
                 self.dependencies.values().map(|deps| deps.len()).sum::<usize>());
    }
}

fn main() -> Result<()> {
    let output2_path = "output2/wrapped-split-decls-rs/src/decls";
    let mut lattice = RustLattice::new();
    
    println!("🔍 Building Rust dependency lattice from {} files...", 14715);
    
    // Walk through declaration files and extract uses
    fn walk_and_extract(dir: &Path, base: &Path, lattice: &mut RustLattice) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension() == Some("rs".as_ref()) {
                    let relative_path = path.strip_prefix(base)?;
                    let name = relative_path.to_string_lossy().replace("/", "_").replace(".rs", "");
                    
                    if let Ok(uses) = lattice.extract_uses_from_file(&path) {
                        lattice.add_node(name, relative_path.to_string_lossy().to_string(), uses);
                    }
                } else if path.is_dir() {
                    walk_and_extract(&path, base, lattice)?;
                }
            }
        }
        Ok(())
    }
    
    let base_path = Path::new(output2_path);
    if base_path.exists() {
        walk_and_extract(base_path, base_path, &mut lattice)?;
        
        println!("✅ Extracted {} nodes", lattice.nodes.len());
        println!("🔧 Building lattice levels...");
        
        lattice.build_lattice_levels();
        lattice.print_lattice();
    } else {
        println!("❌ Output2 directory not found: {}", output2_path);
    }
    
    Ok(())
}
