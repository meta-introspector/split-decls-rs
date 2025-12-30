use std::collections::HashMap;
use std::fs;
use syn::{parse_file, Item, UseTree, UsePath, UseGroup, UseGlob, UseRename};

fn main() {
    println!("🔧 Generating mkmod macros from pub use statements");
    
    // Find all wrapped crates with oldlib.rs files
    let output_dir = "output2";
    let wrapped_dirs = fs::read_dir(output_dir).expect("Failed to read output2");
    
    let mut all_pub_uses = HashMap::new();
    
    for entry in wrapped_dirs {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        if path.is_dir() && path.file_name().unwrap().to_str().unwrap().starts_with("wrapped-") {
            let oldlib_path = path.join("src/oldlib.rs");
            if oldlib_path.exists() {
                let crate_name = path.file_name().unwrap().to_str().unwrap();
                println!("📦 Processing {}", crate_name);
                
                let pub_uses = extract_pub_uses(&oldlib_path);
                if !pub_uses.is_empty() {
                    all_pub_uses.insert(crate_name.to_string(), pub_uses);
                }
            }
        }
    }
    
    // Generate mkmod macros
    generate_mkmod_macros(&all_pub_uses);
    println!("✅ Generated mkmod macros for {} crates", all_pub_uses.len());
}

fn extract_pub_uses(oldlib_path: &std::path::Path) -> Vec<String> {
    let content = fs::read_to_string(oldlib_path).expect("Failed to read oldlib.rs");
    let syntax_tree = parse_file(&content).expect("Failed to parse oldlib.rs");
    
    let mut pub_uses = Vec::new();
    
    for item in syntax_tree.items {
        if let Item::Use(use_item) = item {
            if matches!(use_item.vis, syn::Visibility::Public(_)) {
                let use_str = format!("pub use {};", use_tree_to_string(&use_item.tree));
                pub_uses.push(use_str);
            }
        }
    }
    
    pub_uses
}

fn use_tree_to_string(tree: &UseTree) -> String {
    match tree {
        UseTree::Path(UsePath { ident, tree, .. }) => {
            format!("{}::{}", ident, use_tree_to_string(tree))
        }
        UseTree::Name(name) => name.ident.to_string(),
        UseTree::Rename(UseRename { ident, rename, .. }) => {
            format!("{} as {}", ident, rename)
        }
        UseTree::Glob(UseGlob { .. }) => "*".to_string(),
        UseTree::Group(UseGroup { items, .. }) => {
            let items: Vec<String> = items.iter().map(use_tree_to_string).collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}

fn generate_mkmod_macros(all_pub_uses: &HashMap<String, Vec<String>>) {
    let mut output = String::new();
    
    for (crate_name, pub_uses) in all_pub_uses {
        let safe_crate_name = crate_name.replace("-", "_");
        
        output.push_str(&format!("macro_rules! mkmod_{} {{\n", safe_crate_name));
        output.push_str("    () => {\n");
        
        for pub_use in pub_uses {
            // Convert pub use to macro calls
            let macro_call = convert_pub_use_to_macro_call(pub_use);
            output.push_str(&format!("        {};\n", macro_call));
        }
        
        output.push_str("    };\n");
        output.push_str("}\n\n");
    }
    
    fs::write("src/mkmod_macros.rs", output).expect("Failed to write mkmod_macros.rs");
}

fn convert_pub_use_to_macro_call(pub_use: &str) -> String {
    // Extract the used path and convert to import macro call
    // pub use crate::ATerm; -> import_ATerm!()
    // pub use some::path::*; -> import_some_path_all!()
    
    if pub_use.contains("crate::") {
        let parts: Vec<&str> = pub_use.split("crate::").collect();
        if parts.len() > 1 {
            let symbol = parts[1].trim_end_matches(';').trim();
            if symbol == "*" {
                return "import_crate_all!()".to_string();
            } else {
                let safe_symbol = symbol.replace("::", "_");
                return format!("import_{}!()", safe_symbol);
            }
        }
    }
    
    // Fallback: comment out the original
    format!("// {}", pub_use)
}
