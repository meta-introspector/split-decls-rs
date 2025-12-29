use std::collections::HashMap;
use std::fs;
use serde_json;
use syn::visit::Visit;

fn main() {
    println!("🔍 Creating name index from dependency cache...");
    
    // Load the file-path-based cache
    let cache_data = fs::read_to_string("dependency_cache.json").expect("Failed to read dependency_cache.json");
    let cache: serde_json::Value = serde_json::from_str(&cache_data).expect("Failed to parse JSON");
    
    let mut name_index: HashMap<String, String> = HashMap::new();
    
    if let Some(files) = cache.get("files") {
        if let Some(files_obj) = files.as_object() {
            for (file_path, _deps) in files_obj {
                // Parse each file with syn to extract all function names
                if let Ok(names) = extract_names_from_file(file_path) {
                    for name in names {
                        name_index.insert(name, file_path.clone());
                    }
                }
            }
        }
    }
    
    println!("📊 Created index with {} names", name_index.len());
    
    // Save the name index
    let index_json = serde_json::to_string_pretty(&name_index).expect("Failed to serialize");
    fs::write("name_index.json", index_json).expect("Failed to write name_index.json");
    
    println!("✅ Saved name_index.json");
    
    // Test lookups
    println!("🔍 Testing lookups:");
    for term in ["get_resident_set_size", "install_ice_hook", "TimePassesCallbacks", "run_compiler"] {
        if let Some(path) = name_index.get(term) {
            println!("  ✅ {}: {}", term, path);
        } else {
            println!("  ❌ {} not found", term);
        }
    }
}

fn extract_names_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut names = Vec::new();
    
    // Try to parse as Rust code
    if let Ok(parsed) = syn::parse_file(&content) {
        let mut visitor = NameVisitor { names: Vec::new() };
        visitor.visit_file(&parsed);
        names = visitor.names;
    }
    
    Ok(names)
}

struct NameVisitor {
    names: Vec<String>,
}

impl<'ast> Visit<'ast> for NameVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.names.push(node.sig.ident.to_string());
        syn::visit::visit_item_fn(self, node);
    }
    
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.names.push(node.ident.to_string());
        syn::visit::visit_item_struct(self, node);
    }
    
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        self.names.push(node.ident.to_string());
        syn::visit::visit_item_enum(self, node);
    }
    
    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.names.push(node.ident.to_string());
        syn::visit::visit_item_trait(self, node);
    }
    
    fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {
        self.names.push(node.ident.to_string());
        syn::visit::visit_item_const(self, node);
    }
    
    fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {
        self.names.push(node.ident.to_string());
        syn::visit::visit_item_static(self, node);
    }
}
