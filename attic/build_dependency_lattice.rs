use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use serde_json::Value;

#[derive(Debug, Clone)]
struct FunctionNode {
    name: String,
    dependencies: Vec<String>,
    dependents: Vec<String>,
    level: Option<usize>, // Position in lattice
    is_leaf: bool,        // No dependencies (bottom of lattice)
    is_root: bool,        // No dependents (top of lattice)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Building rustc dependency lattice...");
    
    // Load symbol map and analysis
    let symbol_data = fs::read_to_string("symbol_map.json")?;
    let symbol_map: HashMap<String, Value> = serde_json::from_str(&symbol_data)?;
    
    let analysis = fs::read_to_string("rustc_complete_analysis.txt")?;
    
    // Build dependency graph
    let mut nodes = HashMap::new();
    let mut all_functions = HashSet::new();
    
    // Extract all function calls and their dependencies
    for line in analysis.lines() {
        if line.contains("📍") && line.contains("🔗") {
            // Parse function and its calls
            if let Some(func_part) = line.split("📍").nth(1) {
                if let Some(func_name) = func_part.split(":").next() {
                    let func_name = func_name.trim().to_string();
                    all_functions.insert(func_name.clone());
                    
                    let node = nodes.entry(func_name.clone()).or_insert_with(|| FunctionNode {
                        name: func_name.clone(),
                        dependencies: Vec::new(),
                        dependents: Vec::new(),
                        level: None,
                        is_leaf: false,
                        is_root: false,
                    });
                    
                    // Find dependencies in subsequent lines
                    // This is simplified - would need full parsing for complete accuracy
                }
            }
        }
        
        // Extract direct dependencies from call lines
        if line.trim().starts_with("🔄") || line.trim().starts_with("📍") {
            // Parse dependency relationships
        }
    }
    
    // Calculate lattice levels using topological sort
    let levels = calculate_lattice_levels(&mut nodes);
    
    // Generate lattice visualization
    generate_lattice_output(&nodes, levels)?;
    
    Ok(())
}

fn calculate_lattice_levels(nodes: &mut HashMap<String, FunctionNode>) -> Vec<Vec<String>> {
    let mut levels = Vec::new();
    let mut in_degree = HashMap::new();
    let mut queue = VecDeque::new();
    
    // Calculate in-degrees (number of dependencies)
    for (name, node) in nodes.iter() {
        let degree = node.dependencies.len();
        in_degree.insert(name.clone(), degree);
        
        if degree == 0 {
            queue.push_back(name.clone());
            nodes.get_mut(name).unwrap().is_leaf = true;
        }
    }
    
    let mut current_level = 0;
    
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut current_level_nodes = Vec::new();
        
        for _ in 0..level_size {
            if let Some(node_name) = queue.pop_front() {
                current_level_nodes.push(node_name.clone());
                
                if let Some(node) = nodes.get_mut(&node_name) {
                    node.level = Some(current_level);
                    
                    // Process dependents
                    for dependent in &node.dependents.clone() {
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
            levels.push(current_level_nodes);
            current_level += 1;
        }
    }
    
    // Mark root nodes (highest level)
    if let Some(top_level) = levels.last() {
        for node_name in top_level {
            if let Some(node) = nodes.get_mut(node_name) {
                node.is_root = true;
            }
        }
    }
    
    levels
}

fn generate_lattice_output(nodes: &HashMap<String, FunctionNode>, levels: Vec<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = fs::File::create("rustc_dependency_lattice.txt")?;
    use std::io::Write;
    
    writeln!(output, "🔬 RUSTC DEPENDENCY LATTICE")?;
    writeln!(output, "==========================")?;
    writeln!(output, "Total Levels: {}", levels.len())?;
    writeln!(output, "Total Functions: {}", nodes.len())?;
    writeln!(output)?;
    
    for (level_idx, level_nodes) in levels.iter().enumerate() {
        writeln!(output, "📊 LEVEL {} ({} functions)", level_idx, level_nodes.len())?;
        writeln!(output, "├─ Lattice Position: {}", level_idx)?;
        
        if level_idx == 0 {
            writeln!(output, "├─ Type: LEAF NODES (no dependencies)")?;
        } else if level_idx == levels.len() - 1 {
            writeln!(output, "├─ Type: ROOT NODES (rustc::main)")?;
        } else {
            writeln!(output, "├─ Type: INTERMEDIATE NODES")?;
        }
        
        writeln!(output, "└─ Functions:")?;
        
        for (idx, func_name) in level_nodes.iter().enumerate() {
            let prefix = if idx == level_nodes.len() - 1 { "   └─" } else { "   ├─" };
            
            if let Some(node) = nodes.get(func_name) {
                let deps = node.dependencies.len();
                let dependents = node.dependents.len();
                writeln!(output, "{} {} (deps: {}, used_by: {})", prefix, func_name, deps, dependents)?;
            }
        }
        writeln!(output)?;
    }
    
    // Generate macro composition order
    writeln!(output, "🏗️ MACRO COMPOSITION ORDER")?;
    writeln!(output, "=========================")?;
    writeln!(output, "// Build from bottom (leaves) to top (main)")?;
    
    for (level_idx, level_nodes) in levels.iter().enumerate() {
        writeln!(output, "// Level {}: {} functions", level_idx, level_nodes.len())?;
        for func_name in level_nodes {
            let macro_name = func_name.replace("::", "_").replace(".", "_").to_lowercase();
            writeln!(output, "{}!();", macro_name)?;
        }
        writeln!(output)?;
    }
    
    println!("✅ Generated rustc_dependency_lattice.txt");
    println!("📊 Lattice has {} levels with {} total functions", levels.len(), nodes.len());
    
    Ok(())
}
