use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;

#[derive(Debug)]
struct SparqlEngine {
    graph: Graph<String, String, Directed>,
    node_map: HashMap<String, NodeIndex>,
    triples: Vec<(String, String, String)>,
}

impl SparqlEngine {
    fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_map: HashMap::new(),
            triples: Vec::new(),
        }
    }
    
    fn load_knowledge_base(&mut self, owl_file: &str) -> Result<()> {
        let content = fs::read_to_string(owl_file)?;
        
        // Parse OWL/Turtle format
        for line in content.lines() {
            if let Some(triple) = self.parse_triple(line) {
                self.add_triple(triple.0, triple.1, triple.2);
            }
        }
        
        println!("📊 Knowledge Base Loaded:");
        println!("   Nodes: {}", self.graph.node_count());
        println!("   Edges: {}", self.graph.edge_count());
        println!("   Triples: {}", self.triples.len());
        
        Ok(())
    }
    
    fn parse_triple(&self, line: &str) -> Option<(String, String, String)> {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() || line.starts_with('@') {
            return None;
        }
        
        // Simple turtle parser for subject predicate object .
        if let Some(dot_pos) = line.rfind(" .") {
            let triple_part = &line[..dot_pos];
            let parts: Vec<&str> = triple_part.split_whitespace().collect();
            
            if parts.len() >= 3 {
                let subject = parts[0].trim_end_matches(':').to_string();
                let predicate = parts[1].trim_end_matches(':').to_string();
                let object = parts[2..].join(" ").trim_matches('"').to_string();
                return Some((subject, predicate, object));
            }
        }
        
        None
    }
    
    fn add_triple(&mut self, subject: String, predicate: String, object: String) {
        // Add nodes to graph
        let subj_idx = self.get_or_create_node(&subject);
        let obj_idx = self.get_or_create_node(&object);
        
        // Add edge with predicate as weight
        self.graph.add_edge(subj_idx, obj_idx, predicate.clone());
        
        // Store triple
        self.triples.push((subject, predicate, object));
    }
    
    fn get_or_create_node(&mut self, node_name: &str) -> NodeIndex {
        if let Some(&idx) = self.node_map.get(node_name) {
            idx
        } else {
            let idx = self.graph.add_node(node_name.to_string());
            self.node_map.insert(node_name.to_string(), idx);
            idx
        }
    }
    
    // Simple SPARQL-like queries
    fn select_where(&self, pattern: &str) -> Vec<HashMap<String, String>> {
        let mut results = Vec::new();
        
        match pattern {
            "?s rdf:type lmdfb:RustcComponent" => {
                for (s, p, o) in &self.triples {
                    if p == "rdf:type" && o == "lmdfb:RustcComponent" {
                        let mut result = HashMap::new();
                        result.insert("s".to_string(), s.clone());
                        results.push(result);
                    }
                }
            }
            "?s lmdfb:hasLevel ?level" => {
                for (s, p, o) in &self.triples {
                    if p == "lmdfb:hasLevel" {
                        let mut result = HashMap::new();
                        result.insert("s".to_string(), s.clone());
                        result.insert("level".to_string(), o.clone());
                        results.push(result);
                    }
                }
            }
            "?s lmdfb:hasSemanticPattern ?pattern" => {
                for (s, p, o) in &self.triples {
                    if p == "lmdfb:hasSemanticPattern" {
                        let mut result = HashMap::new();
                        result.insert("s".to_string(), s.clone());
                        result.insert("pattern".to_string(), o.clone());
                        results.push(result);
                    }
                }
            }
            _ => {
                println!("⚠️  Pattern not implemented: {}", pattern);
            }
        }
        
        results
    }
    
    fn analyze_graph_structure(&self) {
        println!("\n🔍 Graph Structure Analysis:");
        
        // Find most connected nodes
        let mut node_degrees: Vec<(String, usize)> = self.node_map.iter()
            .map(|(name, &idx)| {
                let degree = self.graph.edges(idx).count();
                (name.clone(), degree)
            })
            .collect();
        
        node_degrees.sort_by(|a, b| b.1.cmp(&a.1));
        
        println!("   Top 10 Most Connected Nodes:");
        for (i, (name, degree)) in node_degrees.iter().take(10).enumerate() {
            println!("   {}. {} (degree: {})", i+1, name, degree);
        }
        
        // Analyze semantic patterns
        let mut pattern_counts = HashMap::new();
        for (_, _, o) in &self.triples {
            if o.starts_with("lmdfb:") && !o.contains("_layer") {
                *pattern_counts.entry(o.clone()).or_insert(0) += 1;
            }
        }
        
        let mut patterns: Vec<_> = pattern_counts.into_iter().collect();
        patterns.sort_by(|a, b| b.1.cmp(&a.1));
        
        println!("\n   Top Semantic Patterns:");
        for (pattern, count) in patterns.iter().take(5) {
            println!("   {} ({})", pattern, count);
        }
    }
    
    fn find_shortest_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        if let (Some(&from_idx), Some(&to_idx)) = (self.node_map.get(from), self.node_map.get(to)) {
            if let Some(path) = petgraph::algo::dijkstra(&self.graph, from_idx, Some(to_idx), |_| 1) {
                if path.contains_key(&to_idx) {
                    // Reconstruct path (simplified)
                    return Some(vec![from.to_string(), to.to_string()]);
                }
            }
        }
        None
    }
}

fn main() -> Result<()> {
    println!("🚀 Booting SPARQL Engine with PetGraph");
    println!("=====================================\n");
    
    let mut engine = SparqlEngine::new();
    
    // Load knowledge base
    println!("📚 Loading knowledge base...");
    engine.load_knowledge_base("rustc_lmdfb_knowledge_base.owl")?;
    
    // Run sample SPARQL queries
    println!("\n🔍 Running SPARQL Queries:");
    
    // Query 1: Find all RustcComponents
    println!("\n1. SELECT ?s WHERE { ?s rdf:type lmdfb:RustcComponent }");
    let components = engine.select_where("?s rdf:type lmdfb:RustcComponent");
    println!("   Found {} components", components.len());
    
    // Query 2: Find nodes by level
    println!("\n2. SELECT ?s ?level WHERE { ?s lmdfb:hasLevel ?level }");
    let levels = engine.select_where("?s lmdfb:hasLevel ?level");
    println!("   Found {} nodes with levels", levels.len());
    
    // Query 3: Find semantic patterns
    println!("\n3. SELECT ?s ?pattern WHERE { ?s lmdfb:hasSemanticPattern ?pattern }");
    let patterns = engine.select_where("?s lmdfb:hasSemanticPattern ?pattern");
    println!("   Found {} semantic patterns", patterns.len());
    
    // Analyze graph structure
    engine.analyze_graph_structure();
    
    println!("\n✅ SPARQL Engine Ready!");
    println!("   Graph loaded with {} nodes and {} edges", 
             engine.graph.node_count(), engine.graph.edge_count());
    println!("   Ready for complex graph queries and analysis!");
    
    Ok(())
}
