use anyhow::Result;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!("🔬 8-Level K-Theory Dependency Analysis");
    
    let decls = load_all_declarations()?;
    println!("📊 Loaded {} declarations", decls.len());
    
    // Build dependency graph
    let dep_graph = build_dependency_graph(&decls)?;
    
    // Analyze K-theory levels (0-7)
    let k_analysis = analyze_k_levels(&dep_graph, &decls)?;
    
    // Report findings
    report_k_theory_analysis(&k_analysis);
    
    Ok(())
}

fn build_dependency_graph(decls: &[Declaration]) -> Result<DependencyGraph> {
    println!("🕸️  Building dependency graph...");
    
    let mut graph = HashMap::new();
    let mut reverse_graph = HashMap::new();
    
    for (i, decl) in decls.iter().enumerate() {
        let deps = extract_dependencies(&decl.content);
        graph.insert(i, deps.clone());
        
        // Build reverse graph for K-theory analysis
        for dep in deps {
            reverse_graph.entry(dep).or_insert_with(Vec::new).push(i);
        }
    }
    
    Ok(DependencyGraph { 
        forward: graph, 
        reverse: reverse_graph,
        node_count: decls.len()
    })
}

fn extract_dependencies(content: &str) -> Vec<usize> {
    // Extract dependency patterns and map to indices
    // This is simplified - in real K-theory we'd use proper AST parsing
    let patterns = [
        "use ", "impl ", "struct ", "enum ", "trait ", "fn ",
        "std::", "crate::", "super::", "self::"
    ];
    
    patterns.iter()
        .enumerate()
        .filter(|(_, pattern)| content.contains(*pattern))
        .map(|(i, _)| i % 100) // Simplified mapping
        .collect()
}

fn analyze_k_levels(graph: &DependencyGraph, decls: &[Declaration]) -> Result<KTheoryAnalysis> {
    println!("🧮 Computing K-theory levels 0-7...");
    
    let mut levels = Vec::new();
    
    for k in 0..8 {
        let level_analysis = compute_k_level(k, graph, decls)?;
        println!("   K{}: {} nodes, depth {}", k, level_analysis.node_count, level_analysis.max_depth);
        levels.push(level_analysis);
    }
    
    Ok(KTheoryAnalysis { levels })
}

fn compute_k_level(k: usize, graph: &DependencyGraph, decls: &[Declaration]) -> Result<KLevel> {
    let nodes_at_level: Vec<usize> = (0..graph.node_count).into_par_iter()
        .filter(|&i| compute_node_k_level(i, graph) == k)
        .collect();
    
    let depths: Vec<usize> = nodes_at_level.par_iter()
        .map(|&i| compute_dependency_depth(i, graph))
        .collect();
    
    let max_depth = depths.iter().max().copied().unwrap_or(0);
    let avg_depth = if depths.is_empty() { 0.0 } else { 
        depths.iter().sum::<usize>() as f64 / depths.len() as f64 
    };
    
    // Analyze complexity at this K-level
    let complexities: Vec<usize> = nodes_at_level.par_iter()
        .map(|&i| calculate_node_complexity(&decls[i].content))
        .collect();
    
    let avg_complexity = if complexities.is_empty() { 0.0 } else {
        complexities.iter().sum::<usize>() as f64 / complexities.len() as f64
    };
    
    // Find critical nodes (high connectivity)
    let critical_nodes: Vec<usize> = nodes_at_level.par_iter()
        .filter(|&&i| {
            let in_degree = graph.reverse.get(&i).map_or(0, |v| v.len());
            let out_degree = graph.forward.get(&i).map_or(0, |v| v.len());
            in_degree + out_degree > 5 // Threshold for criticality
        })
        .copied()
        .collect();
    
    let is_essential = max_depth > 0 && !critical_nodes.is_empty();
    
    Ok(KLevel {
        k,
        node_count: nodes_at_level.len(),
        nodes: nodes_at_level,
        max_depth,
        avg_depth,
        avg_complexity,
        critical_nodes,
        is_essential,
    })
}

fn compute_node_k_level(node: usize, graph: &DependencyGraph) -> usize {
    // Simplified K-theory level computation
    // In real K-theory, this would involve homotopy groups and exact sequences
    let in_degree = graph.reverse.get(&node).map_or(0, |v| v.len());
    let out_degree = graph.forward.get(&node).map_or(0, |v| v.len());
    
    match (in_degree, out_degree) {
        (0, 0) => 0,      // K0: Isolated nodes
        (0, _) => 1,      // K1: Sources
        (_, 0) => 2,      // K2: Sinks  
        (1, 1) => 3,      // K3: Simple chains
        (1, _) | (_, 1) => 4, // K4: Fan in/out
        (2, 2) => 5,      // K5: Balanced nodes
        (_, _) if in_degree + out_degree > 10 => 7, // K7: Highly connected
        _ => 6,           // K6: Complex intermediate
    }
}

fn compute_dependency_depth(start: usize, graph: &DependencyGraph) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut max_depth = 0;
    
    queue.push_back((start, 0));
    visited.insert(start);
    
    while let Some((node, depth)) = queue.pop_front() {
        max_depth = max_depth.max(depth);
        
        if let Some(deps) = graph.forward.get(&node) {
            for &dep in deps {
                if !visited.contains(&dep) {
                    visited.insert(dep);
                    queue.push_back((dep, depth + 1));
                }
            }
        }
    }
    
    max_depth
}

fn calculate_node_complexity(content: &str) -> usize {
    let complexity_indicators = [
        "impl", "trait", "struct", "enum", "fn", "match", "if", "for", "while",
        "generic", "where", "async", "unsafe", "macro", "derive"
    ];
    
    complexity_indicators.par_iter()
        .map(|indicator| content.matches(indicator).count())
        .sum()
}

fn report_k_theory_analysis(analysis: &KTheoryAnalysis) {
    println!("\n🎯 K-Theory Dependency Analysis Results:");
    println!("═══════════════════════════════════════");
    
    for level in &analysis.levels {
        let essential_marker = if level.is_essential { "🔥" } else { "  " };
        println!("{}K{}: {} nodes, max_depth={}, avg_depth={:.1}, complexity={:.1}, critical={}",
                 essential_marker, level.k, level.node_count, level.max_depth, 
                 level.avg_depth, level.avg_complexity, level.critical_nodes.len());
    }
    
    // Find minimum essential levels
    let essential_levels: Vec<_> = analysis.levels.iter()
        .filter(|level| level.is_essential)
        .collect();
    
    println!("\n🔥 Essential K-Levels: {}", essential_levels.len());
    for level in essential_levels {
        println!("   K{}: {} critical nodes, depth {}", 
                 level.k, level.critical_nodes.len(), level.max_depth);
    }
    
    // Compute dependency reduction potential
    let total_nodes: usize = analysis.levels.iter().map(|l| l.node_count).sum();
    let essential_nodes: usize = analysis.levels.iter()
        .filter(|l| l.is_essential)
        .map(|l| l.node_count)
        .sum();
    
    let reduction_potential = if total_nodes > 0 {
        (1.0 - essential_nodes as f64 / total_nodes as f64) * 100.0
    } else { 0.0 };
    
    println!("\n📊 Dependency Optimization:");
    println!("   Total nodes: {}", total_nodes);
    println!("   Essential nodes: {}", essential_nodes);
    println!("   Reduction potential: {:.1}%", reduction_potential);
    
    // Recommend optimal K-level
    let optimal_k = analysis.levels.iter()
        .filter(|l| l.is_essential)
        .max_by_key(|l| l.critical_nodes.len())
        .map(|l| l.k)
        .unwrap_or(3);
    
    println!("   🎯 Recommended max K-level: K{}", optimal_k);
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

#[derive(Debug)]
struct DependencyGraph {
    forward: HashMap<usize, Vec<usize>>,
    reverse: HashMap<usize, Vec<usize>>,
    node_count: usize,
}

#[derive(Debug)]
struct KLevel {
    k: usize,
    node_count: usize,
    nodes: Vec<usize>,
    max_depth: usize,
    avg_depth: f64,
    avg_complexity: f64,
    critical_nodes: Vec<usize>,
    is_essential: bool,
}

#[derive(Debug)]
struct KTheoryAnalysis {
    levels: Vec<KLevel>,
}
