use anyhow::Result;
use petgraph::{Graph, Undirected};
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

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
            let mut content = format!(
                "// Partition {}\n// {} nodes\n// Generated from files:\n",
                i,
                partition.len()
            );
            
            // Add file tracking comments
            for name in partition {
                content.push_str(&format!("// - {}\n", name));
            }
            content.push('\n');
            
            // Generate macro calls that can add attributes
            for name in partition {
                let safe_name = name
                    .replace('/', "_")
                    .replace('-', "_")
                    .replace('.', "_")
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '_')
                    .collect::<String>();
                
                content.push_str(&format!(
                    "partition_module!({}, \"{}\");\n",
                    safe_name, name
                ));
            }
            
            fs::write(format!("partition_{}.rs", i), content)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let target_chunk_size = 9; // Target 7-11 nodes, use 9 as middle
    let node_count = 14716; // We know this from previous run
    let num_partitions = (node_count + target_chunk_size - 1) / target_chunk_size;
    
    let mut partitioner = GraphPartitioner::new(num_partitions);
    
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
    
    // If no dependency map, create nodes from all .rs files in output2
    if partitioner.graph.node_count() == 0 {
        for entry in WalkDir::new("output2").into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(path) = entry.path().to_str() {
                    if path.ends_with(".rs") {
                        // Use the relative path as node name
                        let node_name = path.strip_prefix("output2/").unwrap_or(path);
                        partitioner.add_node(node_name.to_string());
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
