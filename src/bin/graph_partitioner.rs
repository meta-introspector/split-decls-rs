use anyhow::Result;
use petgraph::{Graph, Undirected};
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::fs;

type PartitionGraph = UnGraph<String, ()>;

struct GraphPartitioner {
    graph: PartitionGraph,
    node_map: HashMap<String, NodeIndex>,
    chunks: usize,
}

impl GraphPartitioner {
    fn new(chunks: usize) -> Self {
        Self {
            graph: Graph::new_undirected(),
            node_map: HashMap::new(),
            chunks,
        }
    }

    fn add_node(&mut self, name: String) -> NodeIndex {
        if let Some(&idx) = self.node_map.get(&name) {
            idx
        } else {
            let idx = self.graph.add_node(name.clone());
            self.node_map.insert(name, idx);
            idx
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        let from_idx = self.add_node(from.to_string());
        let to_idx = self.add_node(to.to_string());
        self.graph.add_edge(from_idx, to_idx, ());
    }

    fn partition(&self) -> Vec<Vec<String>> {
        let node_count = self.graph.node_count();
        let chunk_size = (node_count + self.chunks - 1) / self.chunks;
        
        let mut partitions = vec![Vec::new(); self.chunks];
        let mut current_chunk = 0;
        
        for (i, node_idx) in self.graph.node_indices().enumerate() {
            if i > 0 && i % chunk_size == 0 {
                current_chunk = (current_chunk + 1).min(self.chunks - 1);
            }
            
            if let Some(name) = self.graph.node_weight(node_idx) {
                partitions[current_chunk].push(name.clone());
            }
        }
        
        partitions.into_iter().filter(|p| !p.is_empty()).collect()
    }

    fn write_partitions(&self, partitions: &[Vec<String>]) -> Result<()> {
        for (i, partition) in partitions.iter().enumerate() {
            let content = format!(
                "// Partition {}\n// {} nodes\n\n{}\n",
                i,
                partition.len(),
                partition.iter()
                    .map(|name| format!("pub use crate::decls::{};", name))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            
            fs::write(format!("partition_{}.rs", i), content)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut partitioner = GraphPartitioner::new(71);
    
    // Read dependency map if it exists
    if let Ok(content) = fs::read_to_string("dependency_map.rs") {
        // Simple parsing - look for deps.insert lines
        for line in content.lines() {
            if line.contains("deps.insert") {
                // Extract dependencies from the line
                // This is a simplified parser
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start+1..].find('"') {
                        let node = &line[start+1..start+1+end];
                        partitioner.add_node(node.to_string());
                        
                        // Look for dependencies in vec![]
                        if let Some(vec_start) = line.find("vec![") {
                            let vec_content = &line[vec_start+5..];
                            if let Some(vec_end) = vec_content.find(']') {
                                let deps = &vec_content[..vec_end];
                                for dep in deps.split(',') {
                                    let dep = dep.trim().trim_matches('"');
                                    if !dep.is_empty() {
                                        partitioner.add_edge(node, dep);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // If no dependency map, create nodes from decls directory
    if partitioner.graph.node_count() == 0 {
        if let Ok(entries) = fs::read_dir("src/decls") {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".rs") {
                            let node_name = name.trim_end_matches(".rs");
                            partitioner.add_node(node_name.to_string());
                        }
                    }
                }
            }
        }
    }
    
    let partitions = partitioner.partition();
    partitioner.write_partitions(&partitions)?;
    
    println!("✅ Created {} partitions from {} nodes", 
             partitions.len(), 
             partitioner.graph.node_count());
    
    for (i, partition) in partitions.iter().enumerate() {
        println!("Partition {}: {} nodes", i, partition.len());
    }
    
    Ok(())
}
