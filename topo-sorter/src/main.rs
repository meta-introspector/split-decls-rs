use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use petgraph::algo::toposort;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

type DependencyGraph = Graph<String, (), Directed>;

struct DeclSorter {
    graph: DependencyGraph,
    node_map: HashMap<String, NodeIndex>,
}

impl DeclSorter {
    fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_map: HashMap::new(),
        }
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

    fn add_dependency(&mut self, from: &str, to: &str) {
        let from_idx = self.add_decl(from.to_string());
        let to_idx = self.add_decl(to.to_string());
        self.graph.add_edge(to_idx, from_idx, ()); // to -> from (dependency direction)
    }

    fn topological_sort(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sorted = toposort(&self.graph, None)
            .map_err(|_| "Cycle detected in dependency graph")?;
        Ok(sorted.into_iter()
            .map(|idx| self.graph[idx].clone())
            .collect())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Analyzing declaration dependencies for topological sort");
    
    let mut sorter = DeclSorter::new();
    
    // Start with run_bootstrap_mode as entry point
    sorter.add_decl("run_bootstrap_mode".to_string());
    
    // Add known dependencies from the bootstrap function
    sorter.add_dependency("run_bootstrap_mode", "SplitDeclsConfig");
    sorter.add_dependency("run_bootstrap_mode", "PathBuf");
    sorter.add_dependency("run_bootstrap_mode", "run_wrapped_workspace_mode");
    
    // Add more dependencies as we discover them
    sorter.add_dependency("SplitDeclsConfig", "HashMap");
    sorter.add_dependency("SplitDeclsConfig", "serde");
    
    let sorted = sorter.topological_sort()?;
    
    println!("📋 Topological order for bootstrap declarations:");
    for (i, decl) in sorted.iter().enumerate() {
        println!("{}. {}", i + 1, decl);
    }
    
    println!("\n🎯 Bootstrap3 should include declarations in this order!");
    Ok(())
}
