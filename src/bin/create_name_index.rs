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

impl NameVisitor {
    fn extract_from_tokens(&mut self, tokens: &proc_macro2::TokenStream) {
        for token in tokens.clone() {
            match token {
                proc_macro2::TokenTree::Ident(ident) => {
                    self.names.push(ident.to_string());
                }
                proc_macro2::TokenTree::Group(group) => {
                    self.extract_from_tokens(&group.stream());
                }
                _ => {}
            }
        }
    }

    fn extract_from_attributes(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            if let Ok(tokens) = attr.meta.require_path_only() {
                self.names.push(tokens.get_ident().unwrap_or(&syn::Ident::new("unknown", proc_macro2::Span::call_site())).to_string());
            }
            // Also extract from attribute arguments
            match &attr.meta {
                syn::Meta::List(meta_list) => {
                    self.extract_from_tokens(&meta_list.tokens);
                }
                syn::Meta::NameValue(meta_name_value) => {
                    if let syn::Expr::Lit(expr_lit) = &meta_name_value.value {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            // Extract identifiers from string literals in attributes
                            for word in lit_str.value().split_whitespace() {
                                if word.chars().all(|c| c.is_alphanumeric() || c == '_') && !word.is_empty() {
                                    self.names.push(word.to_string());
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl<'ast> Visit<'ast> for NameVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.names.push(node.sig.ident.to_string());
        self.extract_from_attributes(&node.attrs);
        syn::visit::visit_item_fn(self, node);
    }
    
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.names.push(node.ident.to_string());
        self.extract_from_attributes(&node.attrs);
        syn::visit::visit_item_struct(self, node);
    }
    
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        self.names.push(node.ident.to_string());
        self.extract_from_attributes(&node.attrs);
        syn::visit::visit_item_enum(self, node);
    }
    
    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.names.push(node.ident.to_string());
        self.extract_from_attributes(&node.attrs);
        syn::visit::visit_item_trait(self, node);
    }
    
    fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {
        self.names.push(node.ident.to_string());
        self.extract_from_attributes(&node.attrs);
        syn::visit::visit_item_const(self, node);
    }
    
    fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {
        self.names.push(node.ident.to_string());
        self.extract_from_attributes(&node.attrs);
        syn::visit::visit_item_static(self, node);
    }

    fn visit_item_macro(&mut self, node: &'ast syn::ItemMacro) {
        if let Some(ident) = &node.ident {
            self.names.push(ident.to_string());
        }
        self.extract_from_tokens(&node.mac.tokens);
        syn::visit::visit_item_macro(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.extract_from_tokens(&node.tokens);
        syn::visit::visit_macro(self, node);
    }

    fn visit_ident(&mut self, node: &'ast syn::Ident) {
        self.names.push(node.to_string());
        syn::visit::visit_ident(self, node);
    }
}
