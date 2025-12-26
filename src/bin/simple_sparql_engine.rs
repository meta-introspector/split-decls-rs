use std::collections::HashMap;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🚀 Simple SPARQL-like Query Engine");
    println!("==================================\\n");
    
    // Load knowledge base
    println!("📚 Loading knowledge base...");
    let content = fs::read_to_string("rustc_lmdfb_knowledge_base.owl")?;
    
    let mut triples = Vec::new();
    let mut predicates = HashMap::new();
    
    // Parse triples
    for line in content.lines() {
        if let Some(triple) = parse_triple(line) {
            triples.push(triple.clone());
            *predicates.entry(triple.1.clone()).or_insert(0) += 1;
        }
    }
    
    println!("📊 Knowledge Base Loaded:");
    println!("   Triples: {}", triples.len());
    println!("   Unique predicates: {}", predicates.len());
    
    // Run queries
    println!("\\n🔍 Running Queries:");
    
    // Query 1: Count RustcComponents
    let components = triples.iter()
        .filter(|(_, p, o)| p == "rdf:type" && o == "lmdfb:RustcComponent")
        .count();
    println!("1. Components: {}", components);
    
    // Query 2: Count level assignments
    let levels = triples.iter()
        .filter(|(_, p, _)| p == "lmdfb:hasLevel")
        .count();
    println!("2. Level assignments: {}", levels);
    
    // Query 3: Count semantic patterns
    let patterns = triples.iter()
        .filter(|(_, p, _)| p == "lmdfb:hasSemanticPattern")
        .count();
    println!("3. Semantic patterns: {}", patterns);
    
    // Top predicates
    let mut pred_vec: Vec<_> = predicates.into_iter().collect();
    pred_vec.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\\n📈 Top 10 Predicates:");
    for (i, (pred, count)) in pred_vec.iter().take(10).enumerate() {
        println!("   {}. {} ({})", i+1, pred, count);
    }
    
    println!("\\n✅ Query Engine Ready!");
    println!("   Knowledge base successfully analyzed");
    
    Ok(())
}

fn parse_triple(line: &str) -> Option<(String, String, String)> {
    let line = line.trim();
    if line.starts_with('#') || line.is_empty() || line.starts_with('@') {
        return None;
    }
    
    // Simple turtle parser
    if let Some(dot_pos) = line.rfind(" .") {
        let triple_part = &line[..dot_pos];
        let parts: Vec<&str> = triple_part.split_whitespace().collect();
        
        if parts.len() >= 3 {
            let subject = parts[0].to_string();
            let predicate = parts[1].to_string();
            let object = parts[2..].join(" ").trim_matches('"').to_string();
            return Some((subject, predicate, object));
        }
    }
    
    None
}
