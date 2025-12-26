use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct LatticeNode {
    symbol: String,
    emoji: String,
    level: u32,
    weight: f64,
    layer: String,
    dependencies: Vec<String>,
    complexity_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct LmdfbMapping {
    nodes: HashMap<String, LatticeNode>,
    layers: HashMap<String, Vec<String>>,
    total_levels: u32,
    weight_distribution: HashMap<u32, f64>,
}

fn main() -> Result<()> {
    // Load the semantic secretome
    let secretome_data = fs::read_to_string("rustc_semantic_secretome.json")?;
    let secretome: serde_json::Value = serde_json::from_str(&secretome_data)?;
    
    let mut lmdfb = LmdfbMapping {
        nodes: HashMap::new(),
        layers: HashMap::new(),
        total_levels: 0,
        weight_distribution: HashMap::new(),
    };
    
    // Process functions
    if let Some(functions) = secretome["functions"].as_object() {
        for (symbol, emoji) in functions {
            let level = calculate_level(symbol);
            let weight = calculate_weight(symbol);
            let layer = determine_layer(symbol, level);
            
            let node = LatticeNode {
                symbol: symbol.clone(),
                emoji: emoji.as_str().unwrap_or("❓").to_string(),
                level,
                weight,
                layer: layer.clone(),
                dependencies: extract_dependencies(symbol),
                complexity_score: calculate_complexity(symbol),
            };
            
            lmdfb.nodes.insert(symbol.clone(), node);
            lmdfb.layers.entry(layer).or_insert_with(Vec::new).push(symbol.clone());
        }
    }
    
    // Process types
    if let Some(types) = secretome["types"].as_object() {
        for (symbol, emoji) in types {
            let level = calculate_level(symbol);
            let weight = calculate_weight(symbol);
            let layer = determine_layer(symbol, level);
            
            let node = LatticeNode {
                symbol: symbol.clone(),
                emoji: emoji.as_str().unwrap_or("❓").to_string(),
                level,
                weight,
                layer: layer.clone(),
                dependencies: extract_dependencies(symbol),
                complexity_score: calculate_complexity(symbol),
            };
            
            lmdfb.nodes.insert(symbol.clone(), node);
            lmdfb.layers.entry(layer).or_insert_with(Vec::new).push(symbol.clone());
        }
    }
    
    // Calculate statistics
    lmdfb.total_levels = lmdfb.nodes.values().map(|n| n.level).max().unwrap_or(0);
    
    for node in lmdfb.nodes.values() {
        *lmdfb.weight_distribution.entry(node.level).or_insert(0.0) += node.weight;
    }
    
    // Save LMDFB mapping
    fs::write("lmdfb_lattice_mapping.json", serde_json::to_string_pretty(&lmdfb)?)?;
    
    println!("🔗 LMDFB Lattice Mapping Generated:");
    println!("   📊 Total nodes: {}", lmdfb.nodes.len());
    println!("   🏗️ Total layers: {}", lmdfb.layers.len());
    println!("   📈 Max level: {}", lmdfb.total_levels);
    println!("   ⚖️ Weight distribution across {} levels", lmdfb.weight_distribution.len());
    
    // Show layer distribution
    for (layer, nodes) in &lmdfb.layers {
        println!("   🎯 {}: {} nodes", layer, nodes.len());
    }
    
    Ok(())
}

fn calculate_level(symbol: &str) -> u32 {
    // Level based on dependency depth and complexity
    let parts: Vec<&str> = symbol.split("::").collect();
    let base_level = parts.len() as u32;
    
    // Adjust based on symbol type and complexity
    if symbol.contains("rustc_") {
        base_level + 3 // Compiler core is deeper
    } else if symbol.contains("std::") || symbol.contains("core::") {
        base_level + 2 // Standard library
    } else if symbol.contains("alloc") || symbol.contains("hash") {
        base_level + 1 // System level
    } else {
        base_level // Application level
    }
}

fn calculate_weight(symbol: &str) -> f64 {
    // Weight based on importance and usage frequency
    let mut weight = 1.0;
    
    // Core compiler components have higher weight
    if symbol.contains("rustc_") { weight *= 3.0; }
    if symbol.contains("codegen") { weight *= 2.5; }
    if symbol.contains("parse") { weight *= 2.0; }
    if symbol.contains("ast") { weight *= 2.0; }
    
    // System components
    if symbol.contains("alloc") { weight *= 1.8; }
    if symbol.contains("hash") { weight *= 1.5; }
    if symbol.contains("io") { weight *= 1.3; }
    
    // Normalize by symbol length (longer = more specific = lower weight)
    weight / (symbol.len() as f64 / 20.0).max(1.0)
}

fn determine_layer(symbol: &str, level: u32) -> String {
    match level {
        0..=2 => "foundation".to_string(),
        3..=4 => "system".to_string(), 
        5..=6 => "compiler".to_string(),
        7..=8 => "frontend".to_string(),
        _ => "application".to_string(),
    }
}

fn extract_dependencies(symbol: &str) -> Vec<String> {
    // Extract potential dependencies from symbol name
    let mut deps = Vec::new();
    
    if symbol.contains("::") {
        let parts: Vec<&str> = symbol.split("::").collect();
        if parts.len() > 1 {
            deps.push(parts[0].to_string()); // Parent module
        }
    }
    
    deps
}

fn calculate_complexity(symbol: &str) -> f64 {
    // Complexity score based on various factors
    let mut score = 1.0;
    
    // Length complexity
    score += symbol.len() as f64 / 10.0;
    
    // Namespace depth
    score += symbol.matches("::").count() as f64 * 0.5;
    
    // Special complexity indicators
    if symbol.contains("impl") { score += 2.0; }
    if symbol.contains("generic") { score += 1.5; }
    if symbol.contains("macro") { score += 1.0; }
    
    score
}
