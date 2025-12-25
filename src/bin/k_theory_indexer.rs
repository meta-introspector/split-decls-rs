use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KNode {
    pub name: String,
    pub level: u8,
    pub index: i32,
    pub complexity: f64,
    pub depth: u32,
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KIndex {
    pub nodes: HashMap<String, KNode>, // "k7.1" -> KNode
    pub levels: HashMap<u8, Vec<String>>, // 7 -> ["k7.1", "k7.2", ...]
}

impl KIndex {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            levels: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, level: u8, index: i32, name: String, complexity: f64, depth: u32, file_path: String, content: String) {
        let key = format!("k{}.{}", level, index);
        let node = KNode { name, level, index, complexity, depth, file_path, content };
        
        self.nodes.insert(key.clone(), node);
        self.levels.entry(level).or_insert_with(Vec::new).push(key);
    }

    pub fn get(&self, addr: &str) -> Option<&KNode> {
        if addr.contains(".-1") {
            let level: u8 = addr.chars().skip(1).take_while(|c| c.is_numeric()).collect::<String>().parse().ok()?;
            let level_nodes = self.levels.get(&level)?;
            let last_key = level_nodes.last()?;
            return self.nodes.get(last_key);
        }
        self.nodes.get(addr)
    }

    pub fn summarize(&self, addr: &str) -> Option<String> {
        let node = self.get(addr)?;
        Some(format!(
            "K{}.{}: {} (complexity: {:.1}, depth: {})\n{}",
            node.level, node.index, node.name, node.complexity, node.depth,
            node.content.lines().take(5).collect::<Vec<_>>().join("\n")
        ))
    }
}

fn main() -> Result<()> {
    let mut index = KIndex::new();
    
    // Load from k_theory_deps output and build index
    let output2_path = Path::new("output2");
    if !output2_path.exists() {
        println!("No output2 directory found");
        return Ok(());
    }

    // Mock data for K7 level (8 nodes with high complexity)
    let k7_files = ["complex_trait.rs", "deep_impl.rs", "macro_system.rs", "type_resolver.rs", 
                    "dependency_graph.rs", "ast_transformer.rs", "semantic_analyzer.rs", "code_generator.rs"];
    
    for (i, file) in k7_files.iter().enumerate() {
        let content = format!("// High complexity K7 node\npub struct {} {{}}", file.replace(".rs", ""));
        index.add_node(7, (i + 1) as i32, file.to_string(), 6.2 + i as f64 * 0.1, 3, 
                      format!("output2/{}", file), content);
    }

    // Save index
    let index_json = serde_json::to_string_pretty(&index)?;
    fs::write("k_theory_index.json", index_json)?;
    
    println!("🔬 K-Theory Index Generated");
    println!("   K7: {} nodes indexed", k7_files.len());
    println!("   Usage: k7.1, k7.8, k7.-1");
    
    Ok(())
}
