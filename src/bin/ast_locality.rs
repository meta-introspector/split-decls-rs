use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!("🧠 AST Locality Analyzer - 99% Local / 1% Global");
    
    let decls = load_all_declarations()?;
    println!("📊 Loaded {} AST nodes", decls.len());
    
    // Analyze locality patterns
    let locality_analysis = analyze_locality(&decls)?;
    
    // Create sparse similarity matrix (only compute 1% global connections)
    let sparse_matrix = create_sparse_matrix(&decls, &locality_analysis)?;
    
    report_locality_findings(&locality_analysis, &sparse_matrix);
    
    Ok(())
}

fn analyze_locality(decls: &[Declaration]) -> Result<LocalityAnalysis> {
    println!("🔍 Analyzing AST locality patterns...");
    
    let analysis: Vec<NodeLocality> = decls.par_iter().enumerate().map(|(i, decl)| {
        let local_deps = find_local_dependencies(&decl.content);
        let global_deps = find_global_dependencies(&decl.content);
        
        let total_deps = local_deps.len() + global_deps.len();
        let local_ratio = if total_deps > 0 { local_deps.len() as f64 / total_deps as f64 } else { 1.0 };
        
        NodeLocality {
            index: i,
            name: decl.name.clone(),
            local_deps,
            global_deps,
            local_ratio,
            complexity: calculate_ast_complexity(&decl.content),
        }
    }).collect();
    
    Ok(LocalityAnalysis { nodes: analysis })
}

fn find_local_dependencies(content: &str) -> Vec<String> {
    // Local patterns: same module, relative imports, local structs/functions
    let local_patterns = [
        "self::", "super::", "crate::", "use crate", "impl ", "struct ", "fn ", "mod ",
        "let ", "mut ", "match ", "if ", "for ", "while ", "loop "
    ];
    
    local_patterns.par_iter()
        .filter(|pattern| content.contains(*pattern))
        .map(|s| s.to_string())
        .collect()
}

fn find_global_dependencies(content: &str) -> Vec<String> {
    // Global patterns: external crates, std lib, system calls
    let global_patterns = [
        "std::", "extern ", "use std", "use anyhow", "use serde", "use rayon",
        "HashMap", "Vec", "Result", "Option", "Box", "Arc", "Mutex"
    ];
    
    global_patterns.par_iter()
        .filter(|pattern| content.contains(*pattern))
        .map(|s| s.to_string())
        .collect()
}

fn calculate_ast_complexity(content: &str) -> usize {
    // Simple complexity metric based on nesting and constructs
    let complexity_indicators = ["{", "}", "(", ")", "[", "]", "match", "if", "for", "impl"];
    complexity_indicators.par_iter()
        .map(|indicator| content.matches(indicator).count())
        .sum()
}

fn create_sparse_matrix(decls: &[Declaration], analysis: &LocalityAnalysis) -> Result<SparseMatrix> {
    println!("🕸️  Creating sparse similarity matrix (1% global connections)...");
    
    let n = decls.len();
    let global_threshold = (n as f64 * 0.01) as usize; // Only 1% global connections
    
    // Find the most globally connected nodes
    let mut global_nodes: Vec<_> = analysis.nodes.iter()
        .enumerate()
        .filter(|(_, node)| node.local_ratio < 0.5) // Less than 50% local = more global
        .collect();
    
    global_nodes.sort_by(|a, b| a.1.local_ratio.partial_cmp(&b.1.local_ratio).unwrap());
    global_nodes.truncate(global_threshold);
    
    println!("🌐 Identified {} global nodes out of {}", global_nodes.len(), n);
    
    // Compute similarities only for global connections
    let connections: Vec<(usize, usize, f64)> = global_nodes.par_iter()
        .flat_map(|(i, _)| {
            decls.par_iter().enumerate()
                .filter_map(move |(j, _)| {
                    if *i != j {
                        let similarity = calculate_similarity(&decls[*i].content, &decls[j].content);
                        if similarity > 0.3 {
                            Some((*i, j, similarity))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
        })
        .collect();
    
    Ok(SparseMatrix {
        size: n,
        connections,
        global_nodes: global_nodes.into_iter().map(|(i, _)| i).collect(),
    })
}

fn calculate_similarity(content1: &str, content2: &str) -> f64 {
    let tokens1: Vec<&str> = content1.split_whitespace().collect();
    let tokens2: Vec<&str> = content2.split_whitespace().collect();
    
    let common_tokens = tokens1.par_iter()
        .filter(|token| tokens2.contains(token))
        .count();
    
    let total_tokens = (tokens1.len() + tokens2.len()) as f64;
    if total_tokens == 0.0 { 0.0 } else { (2.0 * common_tokens as f64) / total_tokens }
}

fn report_locality_findings(analysis: &LocalityAnalysis, sparse_matrix: &SparseMatrix) {
    println!("\n🎯 AST Locality Analysis Results:");
    println!("═══════════════════════════════════");
    
    let avg_local_ratio: f64 = analysis.nodes.par_iter()
        .map(|node| node.local_ratio)
        .sum::<f64>() / analysis.nodes.len() as f64;
    
    let high_local_nodes = analysis.nodes.par_iter()
        .filter(|node| node.local_ratio > 0.95)
        .count();
    
    let global_nodes = analysis.nodes.par_iter()
        .filter(|node| node.local_ratio < 0.5)
        .count();
    
    println!("📊 Average locality ratio: {:.1}%", avg_local_ratio * 100.0);
    println!("🏠 High locality nodes (>95%): {}", high_local_nodes);
    println!("🌐 Global nodes (<50% local): {}", global_nodes);
    println!("🕸️  Sparse connections: {}", sparse_matrix.connections.len());
    println!("💾 Memory saved vs full matrix: {:.1}%", 
             (1.0 - sparse_matrix.connections.len() as f64 / (analysis.nodes.len() * analysis.nodes.len()) as f64) * 100.0);
    
    println!("\n🔝 Most Global Nodes:");
    let mut global_sorted = analysis.nodes.clone();
    global_sorted.par_sort_by(|a, b| a.local_ratio.partial_cmp(&b.local_ratio).unwrap());
    
    for node in global_sorted.iter().take(5) {
        println!("   {:.1}% local - {} (complexity: {})", 
                 node.local_ratio * 100.0, 
                 node.name.chars().take(40).collect::<String>(),
                 node.complexity);
    }
    
    println!("\n🏠 Most Local Nodes:");
    global_sorted.reverse();
    for node in global_sorted.iter().take(5) {
        println!("   {:.1}% local - {} (complexity: {})", 
                 node.local_ratio * 100.0, 
                 node.name.chars().take(40).collect::<String>(),
                 node.complexity);
    }
}

fn load_all_declarations() -> Result<Vec<Declaration>> {
    let mut decls = Vec::new();
    if Path::new("output2").exists() {
        load_declarations_recursive("output2", &mut decls)?;
    }
    Ok(decls)
}

fn load_declarations_recursive(dir: &str, decls: &mut Vec<Declaration>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            load_declarations_recursive(&path.to_string_lossy(), decls)?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(&path)?;
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            decls.push(Declaration { name, content });
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Declaration {
    name: String,
    content: String,
}

#[derive(Debug, Clone)]
struct NodeLocality {
    index: usize,
    name: String,
    local_deps: Vec<String>,
    global_deps: Vec<String>,
    local_ratio: f64,
    complexity: usize,
}

#[derive(Debug)]
struct LocalityAnalysis {
    nodes: Vec<NodeLocality>,
}

#[derive(Debug)]
struct SparseMatrix {
    size: usize,
    connections: Vec<(usize, usize, f64)>,
    global_nodes: Vec<usize>,
}
