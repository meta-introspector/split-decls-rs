use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{visit::Visit, Item, Ident};
use walkdir::WalkDir;

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
    
    let output2_path = Path::new("output2");
    let mut nodes: HashMap<String, DeclNode> = HashMap::new();
    
    // Step 1: Load all files into hash
    load_all_declarations(&output2_path, &mut nodes)?;
    
    // Step 2: Extract tokens and build dependency graph
    analyze_dependencies(&mut nodes)?;
    
    // Step 3: Show what defines what and what uses what
    show_dependency_analysis(&nodes)?;
    
    // Step 4: Order the dependencies
    let ordered = topological_sort(&nodes)?;
    
    println!("\n📋 Dependency Order ({} nodes):", ordered.len());
    for (i, name) in ordered.iter().enumerate() {
        println!("  {}: {}", i + 1, name);
    }
    
    // Step 5: Save dependency data for build.rs
    save_dependency_data(&nodes, &ordered)?;
    
    Ok(())
}

fn save_dependency_data(nodes: &HashMap<String, DeclNode>, ordered: &[String]) -> Result<()> {
    println!("\n💾 Saving dependency data...");
    
    // Create dependency map for each node
    let mut dep_map = HashMap::new();
    for (name, node) in nodes {
        let mut deps = Vec::new();
        for used_token in &node.uses {
            // Find providers of this token
            for (provider_name, provider_node) in nodes {
                if provider_node.defines.contains(used_token) && provider_name != name {
                    deps.push(provider_name.clone());
                    break;
                }
            }
        }
        dep_map.insert(name.clone(), deps);
    }
    
    // Generate Rust code for the macro
    let mut macro_content = String::new();
    macro_content.push_str("// Auto-generated dependency data\n");
    macro_content.push_str("use std::collections::HashMap;\n\n");
    
    macro_content.push_str("#[macro_export]\n");
    macro_content.push_str("macro_rules! dependency_data {\n");
    macro_content.push_str("    () => {{\n");
    macro_content.push_str("        {\n");
    macro_content.push_str("            let mut deps: HashMap<&'static str, Vec<&'static str>> = HashMap::new();\n");
    
    for (name, deps) in &dep_map {
        if !deps.is_empty() {
            macro_content.push_str(&format!(
                "            deps.insert(\"{}\", vec![{}]);\n",
                name,
                deps.iter().map(|d| format!("\"{}\"", d)).collect::<Vec<_>>().join(", ")
            ));
        }
    }
    
    macro_content.push_str("            deps\n");
    macro_content.push_str("        }\n");
    macro_content.push_str("    }};\n");
    macro_content.push_str("}\n\n");
    
    macro_content.push_str("#[macro_export]\n");
    macro_content.push_str("macro_rules! build_order {\n");
    macro_content.push_str("    () => {\n");
    macro_content.push_str("        vec![\n");
    for name in ordered {
        macro_content.push_str(&format!("            \"{}\",\n", name));
    }
    macro_content.push_str("        ]\n");
    macro_content.push_str("    };\n");
    macro_content.push_str("}\n");
    
    fs::write("dependency_data.rs", macro_content)?;
    println!("✅ Saved dependency data to dependency_data.rs");
    
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
    for entry in WalkDir::new(decls_path) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let path = entry.path().to_path_buf();
            let content = fs::read_to_string(&path)?;
            
            // Extract name from path structure
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
            *file_count += 1;
        }
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

fn topological_sort(nodes: &HashMap<String, DeclNode>) -> Result<Vec<String>> {
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    // Build dependency graph
    for (name, node) in nodes {
        in_degree.insert(name.clone(), 0);
        graph.insert(name.clone(), Vec::new());
    }
    
    for (name, node) in nodes {
        for used_token in &node.uses {
            // Find who provides this token
            for (provider_name, provider_node) in nodes {
                if provider_node.defines.contains(used_token) {
                    graph.get_mut(provider_name).unwrap().push(name.clone());
                    *in_degree.get_mut(name).unwrap() += 1;
                }
            }
        }
    }
    
    // Kahn's algorithm
    let mut queue: Vec<String> = in_degree.iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(name, _)| name.clone())
        .collect();
    
    let mut result = Vec::new();
    
    while let Some(node) = queue.pop() {
        result.push(node.clone());
        
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                let degree = in_degree.get_mut(neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push(neighbor.clone());
                }
            }
        }
    }
    
    Ok(result)
}
