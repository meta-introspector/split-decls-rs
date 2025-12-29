use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{visit::Visit, Item, Ident};
use walkdir::WalkDir;
use serde_json;
use petgraph::{Graph, Undirected};
use petgraph::algo::connected_components;

#[derive(Debug, Clone)]
struct DeclNode {
    path: PathBuf,
    name: String,
    decl_type: String,
    content: String,
    defines: HashSet<String>,  // tokens this node defines
    uses: HashSet<String>,     // tokens this node uses
}

struct TokenVisitor {
    uses: HashSet<String>,
}

impl<'ast> Visit<'ast> for TokenVisitor {
    fn visit_ident(&mut self, ident: &'ast Ident) {
        self.uses.insert(ident.to_string());
    }
}

fn main() -> Result<()> {
    println!("🔍 Analyzing dependency graph from output2...");
    println!("📊 STATUS: Starting dependency analysis");
    println!("📂 Scanning output2 directory for all declarations...");
    
    let output2_path = Path::new("output2");
    let mut nodes: HashMap<String, DeclNode> = HashMap::new();
    
    println!("📊 STATUS: Step 1 - Loading all files into hash table");
    // Step 1: Load all files into hash
    load_all_declarations(&output2_path, &mut nodes)?;
    
    println!("📊 STATUS: Step 2 - Extracting tokens and building dependency graph");
    // Step 2: Extract tokens and build dependency graph
    analyze_dependencies(&mut nodes)?;
    
    println!("📊 STATUS: Step 2.5 - Main-driven dependency analysis");
    // Step 2.5: Main-driven analysis
    analyze_main_driven_dependencies(&nodes)?;
    
    println!("📊 STATUS: Step 3 - Showing dependency analysis");
    // Step 3: Show what defines what and what uses what
    show_dependency_analysis(&nodes)?;
    
    println!("📊 STATUS: Step 4 - Ordering dependencies with chunking");
    // Step 4: Order the dependencies with chunking
    let (ordered, chunks) = topological_sort_with_chunks(&nodes)?;
    
    println!("\n📋 Dependency Order ({} nodes in {} chunks):", ordered.len(), chunks.len());
    for (chunk_idx, chunk) in chunks.iter().enumerate() {
        println!("  Chunk {}: {} nodes", chunk_idx + 1, chunk.len());
        for (i, name) in chunk.iter().take(5).enumerate() {
            println!("    {}: {}", i + 1, name);
        }
        if chunk.len() > 5 {
            println!("    ... and {} more", chunk.len() - 5);
        }
    }
    
    // Step 5: Save dependency data for build.rs
    save_dependency_data(&nodes, &ordered, &chunks)?;
    
    Ok(())
}

fn analyze_main_driven_dependencies(nodes: &HashMap<String, DeclNode>) -> Result<()> {
    println!("🎯 Analyzing main-driven dependencies...");
    
    // 1. Find all main functions
    let mains: Vec<&DeclNode> = nodes.values()
        .filter(|node| node.name == "main" || node.name.contains("main"))
        .collect();
    
    println!("📍 Found {} main functions", mains.len());
    
    // 2. Extract what each main needs
    let mut main_deps: HashMap<String, HashSet<String>> = HashMap::new();
    for main_node in &mains {
        let deps = extract_main_dependencies(main_node, nodes);
        let path_str = main_node.path.to_string_lossy().to_string();
        main_deps.insert(path_str.clone(), deps);
        println!("🔧 Main at {} needs {} dependencies", 
                main_node.path.display(), main_deps[&path_str].len());
    }
    
    // 3. Cluster the libraries used by mains
    let lib_clusters = cluster_main_libraries(&main_deps, nodes);
    
    // 4. Label clusters by main usage
    label_clusters_by_mains(&lib_clusters, &main_deps);
    
    Ok(())
}

fn extract_main_dependencies(main_node: &DeclNode, all_nodes: &HashMap<String, DeclNode>) -> HashSet<String> {
    let mut deps = HashSet::new();
    let mut to_visit = vec![main_node.name.clone()];
    let mut visited = HashSet::new();
    
    while let Some(current) = to_visit.pop() {
        if visited.contains(&current) { continue; }
        visited.insert(current.clone());
        
        if let Some(node) = all_nodes.get(&current) {
            for used in &node.uses {
                if !visited.contains(used) {
                    deps.insert(used.clone());
                    to_visit.push(used.clone());
                }
            }
        }
    }
    
    deps
}

fn cluster_main_libraries(main_deps: &HashMap<String, HashSet<String>>, nodes: &HashMap<String, DeclNode>) -> Vec<Vec<String>> {
    println!("🔗 Clustering libraries used by mains...");
    
    // Collect all libraries used by any main
    let mut all_libs: HashSet<String> = HashSet::new();
    for deps in main_deps.values() {
        all_libs.extend(deps.iter().cloned());
    }
    
    // Create petgraph for library clustering
    let mut graph = Graph::<String, (), Undirected>::new_undirected();
    let mut node_indices = HashMap::new();
    
    for lib in &all_libs {
        let idx = graph.add_node(lib.clone());
        node_indices.insert(lib.clone(), idx);
    }
    
    // Add edges between libraries that are used together
    for lib1 in &all_libs {
        for lib2 in &all_libs {
            if lib1 != lib2 {
                if let (Some(node1), Some(node2)) = (nodes.get(lib1), nodes.get(lib2)) {
                    if !node1.uses.is_disjoint(&node2.uses) {
                        if let (Some(&idx1), Some(&idx2)) = (node_indices.get(lib1), node_indices.get(lib2)) {
                            graph.add_edge(idx1, idx2, ());
                        }
                    }
                }
            }
        }
    }
    
    // Simple clustering - can be enhanced
    let components = connected_components(&graph);
    println!("📊 Found {} library clusters", components);
    
    vec![] // Placeholder - implement actual clustering
}

fn label_clusters_by_mains(clusters: &[Vec<String>], main_deps: &HashMap<String, HashSet<String>>) {
    println!("🏷️ Labeling clusters by main usage...");
    
    for (i, cluster) in clusters.iter().enumerate() {
        let mut used_by_mains = Vec::new();
        
        for (main_path, deps) in main_deps {
            if cluster.iter().any(|lib| deps.contains(lib)) {
                used_by_mains.push(main_path.clone());
            }
        }
        
        println!("📦 Cluster {}: {} libs, used by {} mains", 
                i, cluster.len(), used_by_mains.len());
    }
}

fn save_dependency_data(nodes: &HashMap<String, DeclNode>, ordered: &[String], chunks: &[Vec<String>]) -> Result<()> {
    println!("\n💾 Saving dependency data...");
    
    // Save chunks as binary JSON instead of massive text file
    let chunk_data = serde_json::json!({
        "total_nodes": ordered.len(),
        "chunk_count": chunks.len(),
        "chunks": chunks.iter().enumerate().map(|(i, chunk)| {
            serde_json::json!({
                "level": i,
                "count": chunk.len(),
                "nodes": chunk.iter().take(100).collect::<Vec<_>>() // Only save first 100 per chunk
            })
        }).collect::<Vec<_>>()
    });
    
    fs::write("dependency_chunks.json", serde_json::to_string_pretty(&chunk_data)?)?;
    println!("✅ Saved {} chunks to dependency_chunks.json", chunks.len());
    
    // Save summary statistics only
    let summary = format!(
        "// Dependency Analysis Summary\n\
         // Total nodes: {}\n\
         // Dependency levels: {}\n\
         // Parallel potential: {:.2}%\n\
         \n\
         pub const TOTAL_NODES: usize = {};\n\
         pub const DEPENDENCY_LEVELS: usize = {};\n\
         pub const PARALLEL_NODES: usize = {};\n",
        ordered.len(),
        chunks.len(),
        (chunks[0].len() as f64 / ordered.len() as f64) * 100.0,
        ordered.len(),
        chunks.len(),
        chunks[0].len()
    );
    
    fs::write("dependency_summary.rs", summary)?;
    println!("✅ Saved summary to dependency_summary.rs");
    
    Ok(())
}

fn load_all_declarations(output2_path: &Path, nodes: &mut HashMap<String, DeclNode>) -> Result<()> {
    if !output2_path.exists() {
        println!("❌ output2 directory not found");
        return Ok(());
    }
    
    let mut file_count = 0;
    
    for entry in fs::read_dir(output2_path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let crate_path = entry.path();
            let decls_path = crate_path.join("src/decls");
            
            if decls_path.exists() {
                load_crate_declarations(&decls_path, nodes, &mut file_count)?;
            }
        }
    }
    
    println!("📊 Loaded {} declaration files", file_count);
    Ok(())
}

fn load_crate_declarations(decls_path: &Path, nodes: &mut HashMap<String, DeclNode>, file_count: &mut usize) -> Result<()> {
    let mut processed = 0;
    let mut skipped = 0;
    
    for entry in WalkDir::new(decls_path) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let path = entry.path().to_path_buf();
            
            // Progress reporting every 100 files
            if processed % 100 == 0 {
                println!("📊 Processing file {}: {}", processed, path.display());
            }
            
            // Skip if already processed (resumable)
            let path_str = path.to_string_lossy().to_string();
            if nodes.contains_key(&path_str) {
                skipped += 1;
                continue;
            }
            
            match fs::read_to_string(&path) {
                Ok(content) => {
                    processed += 1;
                    *file_count += 1;
                    
                    let name = extract_name_from_path(&path);
                    let decl_type = extract_type_from_path(&path);
                    
                    let node = DeclNode {
                        path: path.clone(),
                        name: name.clone(),
                        decl_type,
                        content,
                        defines: HashSet::new(),
                        uses: HashSet::new(),
                    };
                    
                    nodes.insert(name, node);
                }
                Err(_) => {
                    println!("⚠️  Failed to read: {}", path.display());
                }
            }
        }
    }
    
    if processed > 0 {
        println!("✅ Processed {} files, skipped {}", processed, skipped);
    }
    Ok(())
}

fn extract_name_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn extract_type_from_path(path: &Path) -> String {
    path.parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn analyze_dependencies(nodes: &mut HashMap<String, DeclNode>) -> Result<()> {
    println!("🔍 Analyzing dependencies...");
    
    for (name, node) in nodes.iter_mut() {
        // Parse the content to extract what this node defines and uses
        if let Ok(parsed) = syn::parse_file(&node.content) {
            // What this node defines
            for item in &parsed.items {
                match item {
                    Item::Fn(f) => { node.defines.insert(f.sig.ident.to_string()); }
                    Item::Struct(s) => { node.defines.insert(s.ident.to_string()); }
                    Item::Enum(e) => { node.defines.insert(e.ident.to_string()); }
                    Item::Trait(t) => { node.defines.insert(t.ident.to_string()); }
                    Item::Type(t) => { node.defines.insert(t.ident.to_string()); }
                    Item::Const(c) => { node.defines.insert(c.ident.to_string()); }
                    Item::Static(s) => { node.defines.insert(s.ident.to_string()); }
                    _ => {}
                }
            }
            
            // What this node uses
            let mut visitor = TokenVisitor { uses: HashSet::new() };
            visitor.visit_file(&parsed);
            
            // Filter out common keywords and self-references
            for token in visitor.uses {
                if !is_keyword(&token) && !node.defines.contains(&token) {
                    node.uses.insert(token);
                }
            }
        }
        
        println!("  📦 {}: defines {} tokens, uses {} tokens", 
                 name, node.defines.len(), node.uses.len());
    }
    
    Ok(())
}

fn is_keyword(token: &str) -> bool {
    matches!(token, 
        "fn" | "struct" | "enum" | "trait" | "impl" | "pub" | "use" | "mod" | 
        "let" | "mut" | "const" | "static" | "if" | "else" | "match" | "for" | 
        "while" | "loop" | "return" | "break" | "continue" | "true" | "false" |
        "self" | "Self" | "super" | "crate" | "std" | "String" | "Vec" | "Option" |
        "Result" | "Some" | "None" | "Ok" | "Err" | "println" | "format"
    )
}

fn show_dependency_analysis(nodes: &HashMap<String, DeclNode>) -> Result<()> {
    println!("\n📋 Dependency Analysis:");
    
    // Build reverse lookup: what defines each token
    let mut token_providers: HashMap<String, Vec<String>> = HashMap::new();
    for (name, node) in nodes {
        for token in &node.defines {
            token_providers.entry(token.clone()).or_default().push(name.clone());
        }
    }
    
    // Show dependencies
    for (name, node) in nodes {
        if !node.uses.is_empty() {
            println!("\n🔗 {} uses:", name);
            for token in &node.uses {
                if let Some(providers) = token_providers.get(token) {
                    println!("    {} ← {}", token, providers.join(", "));
                } else {
                    println!("    {} ← (external)", token);
                }
            }
        }
    }
    
    Ok(())
}

fn topological_sort_with_chunks(nodes: &HashMap<String, DeclNode>) -> Result<(Vec<String>, Vec<Vec<String>>)> {
    println!("🔄 Building dependency graph with {} nodes...", nodes.len());
    
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    // Build dependency graph
    for (name, _node) in nodes {
        in_degree.insert(name.clone(), 0);
        graph.insert(name.clone(), Vec::new());
    }
    
    let mut edge_count = 0;
    for (name, node) in nodes {
        for used_token in &node.uses {
            // Find who provides this token
            for (provider_name, provider_node) in nodes {
                if provider_node.defines.contains(used_token) && provider_name != name {
                    graph.get_mut(provider_name).unwrap().push(name.clone());
                    *in_degree.get_mut(name).unwrap() += 1;
                    edge_count += 1;
                }
            }
        }
    }
    
    println!("📊 Graph built: {} edges between {} nodes", edge_count, nodes.len());
    
    // Kahn's algorithm with level-based chunking
    let mut ordered = Vec::new();
    let mut chunks = Vec::new();
    let mut current_level = 0;
    
    let mut queue: Vec<String> = in_degree.iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(name, _)| name.clone())
        .collect();
    
    println!("🚀 Starting with {} root nodes (no dependencies)", queue.len());
    
    while !queue.is_empty() {
        let mut next_queue = Vec::new();
        let mut current_chunk = Vec::new();
        
        // Process all nodes at current level
        for node in queue.drain(..) {
            ordered.push(node.clone());
            current_chunk.push(node.clone());
            
            // Update dependencies
            if let Some(dependents) = graph.get(&node) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            next_queue.push(dependent.clone());
                        }
                    }
                }
            }
        }
        
        if !current_chunk.is_empty() {
            println!("📦 Level {}: {} nodes ready for parallel processing", current_level, current_chunk.len());
            chunks.push(current_chunk);
            current_level += 1;
        }
        
        queue = next_queue;
    }
    
    if ordered.len() != nodes.len() {
        println!("⚠️  Cycle detected! Ordered {} of {} nodes", ordered.len(), nodes.len());
    } else {
        println!("✅ Successfully ordered all {} nodes in {} levels", ordered.len(), chunks.len());
    }
    
    Ok((ordered, chunks))
}
    

