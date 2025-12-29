use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use serde_json;
use syn::visit::Visit;

fn main() {
    println!("🔄 Starting recursive dependency resolution...");
    
    // Load name index
    let name_index: HashMap<String, String> = serde_json::from_str(
        &fs::read_to_string("name_index.json").expect("Failed to read name_index.json")
    ).expect("Failed to parse name_index.json");
    
    // Starting terms from rustc main function
    let initial_terms = vec![
        "get_resident_set_size", "TimePassesCallbacks", "install_ice_hook", 
        "run_compiler", "main", "init_rustc_env_logger", "catch_with_exit_code"
    ];
    
    let mut resolved = HashSet::new();
    let mut queue = VecDeque::new();
    let mut file_dependencies = HashMap::new();
    
    // Initialize queue
    for term in initial_terms {
        queue.push_back(term.to_string());
    }
    
    println!("🔍 Resolving dependencies recursively...");
    
    while let Some(current_term) = queue.pop_front() {
        if resolved.contains(&current_term) {
            continue;
        }
        
        resolved.insert(current_term.clone());
        
        if let Some(file_path) = name_index.get(&current_term) {
            println!("  ✅ {}: {}", current_term, file_path);
            
            // Extract dependencies from this file
            if let Ok(deps) = extract_dependencies_from_file(file_path) {
                file_dependencies.insert(current_term.clone(), deps.clone());
                
                // Add new dependencies to queue
                for dep in deps {
                    if !resolved.contains(&dep) {
                        queue.push_back(dep);
                    }
                }
            }
        } else {
            println!("  ❌ {} not found", current_term);
        }
        
        if resolved.len() % 100 == 0 {
            println!("📊 Resolved {} dependencies so far...", resolved.len());
        }
    }
    
    println!("📊 Final results:");
    println!("  Total resolved: {}", resolved.len());
    println!("  Files analyzed: {}", file_dependencies.len());
    
    // Save results
    let output = serde_json::json!({
        "resolved_terms": resolved.into_iter().collect::<Vec<_>>(),
        "file_dependencies": file_dependencies
    });
    
    fs::write("recursive_dependencies.json", serde_json::to_string_pretty(&output).unwrap())
        .expect("Failed to write results");
    
    println!("✅ Saved recursive_dependencies.json");
}

fn extract_dependencies_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut deps = Vec::new();
    
    if let Ok(parsed) = syn::parse_file(&content) {
        let mut visitor = DependencyVisitor { dependencies: Vec::new() };
        visitor.visit_file(&parsed);
        deps = visitor.dependencies;
    }
    
    Ok(deps)
}

struct DependencyVisitor {
    dependencies: Vec<String>,
}

impl DependencyVisitor {
    fn extract_from_tokens(&mut self, tokens: &proc_macro2::TokenStream) {
        for token in tokens.clone() {
            match token {
                proc_macro2::TokenTree::Ident(ident) => {
                    let name = ident.to_string();
                    if name.len() > 2 && !name.starts_with('_') {
                        self.dependencies.push(name);
                    }
                }
                proc_macro2::TokenTree::Group(group) => {
                    self.extract_from_tokens(&group.stream());
                }
                _ => {}
            }
        }
    }
}

impl<'ast> Visit<'ast> for DependencyVisitor {
    fn visit_ident(&mut self, node: &'ast syn::Ident) {
        let name = node.to_string();
        if name.len() > 2 && !name.starts_with('_') && !["self", "Self", "super", "crate"].contains(&name.as_str()) {
            self.dependencies.push(name);
        }
        syn::visit::visit_ident(self, node);
    }
    
    fn visit_item_macro(&mut self, node: &'ast syn::ItemMacro) {
        self.extract_from_tokens(&node.mac.tokens);
        syn::visit::visit_item_macro(self, node);
    }
    
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.extract_from_tokens(&node.tokens);
        syn::visit::visit_macro(self, node);
    }
}
