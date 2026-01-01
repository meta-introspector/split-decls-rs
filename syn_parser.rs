use std::fs;
use std::collections::HashMap;
use syn::{parse_file, Item, UseTree, UsePath, UseGroup};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
struct ModuleDecls {
    types: HashMap<String, String>,
}

impl ModuleDecls {
    fn new() -> Self {
        Self { types: HashMap::new() }
    }
    
    fn merge(&mut self, other: ModuleDecls) {
        for (name, decl) in other.types {
            self.types.insert(name, decl);
        }
    }
    
    fn generate_stub(&self, module_name: &str) -> String {
        let mut stub = format!("pub mod {} {{\n", module_name);
        for decl in self.types.values() {
            stub.push_str(&format!("    {}\n", decl));
        }
        stub.push_str("}\n\n");
        stub
    }
}

fn main() -> Result<()> {
    let file = "src/processed_rustc_abi_rustc_abi_src_callconv.rs";
    let content = fs::read_to_string(file)?;
    
    println!("🔍 Parsing {} with syn...", file);
    
    let ast = parse_file(&content)?;
    let mut accumulated_decls: HashMap<String, ModuleDecls> = HashMap::new();
    
    // Extract module dependencies using syn
    for item in &ast.items {
        match item {
            Item::Use(use_item) => {
                let (module, types) = extract_use_info(&use_item.tree);
                if !module.is_empty() && !types.is_empty() {
                    println!("📦 Found use: {} -> {:?}", module, types);
                    
                    let mut module_decls = ModuleDecls::new();
                    for type_name in types {
                        let decl = format!("#[derive(Copy, Clone, Debug, PartialEq)]\npub struct {};", type_name);
                        module_decls.types.insert(type_name, decl);
                    }
                    
                    accumulated_decls.entry(module)
                        .or_insert_with(ModuleDecls::new)
                        .merge(module_decls);
                }
            }
            Item::Mod(mod_item) => {
                let module_name = mod_item.ident.to_string();
                println!("📁 Found mod: {}", module_name);
                
                // Create default stub for mod declarations
                let mut module_decls = ModuleDecls::new();
                let decl = "#[derive(Copy, Clone, Debug, PartialEq)]\npub struct Stub;".to_string();
                module_decls.types.insert("Stub".to_string(), decl);
                
                accumulated_decls.entry(module_name)
                    .or_insert_with(ModuleDecls::new)
                    .merge(module_decls);
            }
            _ => {}
        }
    }
    
    // Generate complete declarations
    let mut complete_decls = String::new();
    for (module_name, decls) in &accumulated_decls {
        complete_decls.push_str(&decls.generate_stub(module_name));
    }
    
    fs::write("syn_parsed_decls.rs", &complete_decls)?;
    println!("✅ Generated syn_parsed_decls.rs");
    
    // Show what we found
    println!("\n🎯 EXTRACTED DECLARATIONS:");
    for (module_name, decls) in &accumulated_decls {
        println!("📦 Module '{}': {} types", module_name, decls.types.len());
        for type_name in decls.types.keys() {
            println!("   - {}", type_name);
        }
    }
    
    Ok(())
}

fn extract_use_info(tree: &UseTree) -> (String, Vec<String>) {
    match tree {
        UseTree::Path(UsePath { ident, tree, .. }) => {
            let module = ident.to_string();
            let (_, types) = extract_use_info(tree);
            (module, types)
        }
        UseTree::Group(UseGroup { items, .. }) => {
            let mut types = Vec::new();
            for item in items {
                let (_, mut item_types) = extract_use_info(item);
                types.append(&mut item_types);
            }
            (String::new(), types)
        }
        UseTree::Name(name) => {
            (String::new(), vec![name.ident.to_string()])
        }
        _ => (String::new(), Vec::new()),
    }
}
