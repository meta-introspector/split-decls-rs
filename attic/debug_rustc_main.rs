use split_decls_genesis::symbol_resolver::SymbolVisitor;
use syn::{visit::Visit, File};
use std::fs;

fn main() {
    let rustc_main_path = "../rust/compiler/rustc/src/main.rs";
    
    println!("🔍 Debugging rustc main.rs parsing...");
    
    let content = fs::read_to_string(rustc_main_path)
        .expect("Failed to read rustc main.rs");
    
    let syntax_tree: File = syn::parse_file(&content)
        .expect("Failed to parse rustc main.rs");
    
    let mut visitor = SymbolVisitor::new("rustc".to_string());
    visitor.set_file(rustc_main_path.to_string());
    visitor.visit_file(&syntax_tree);
    
    println!("📊 Found {} symbols in rustc main.rs:", visitor.symbols.len());
    for symbol in &visitor.symbols {
        println!("  - {}: {} ({})", symbol.name, symbol.symbol_type, symbol.source_file);
    }
    
    // Check if main function exists
    let main_symbols: Vec<_> = visitor.symbols.iter()
        .filter(|s| s.name == "main")
        .collect();
    
    if main_symbols.is_empty() {
        println!("❌ No main function found!");
        
        // Let's examine the AST structure
        println!("\n🔍 AST items in rustc main.rs:");
        for item in &syntax_tree.items {
            match item {
                syn::Item::Fn(func) => {
                    println!("  - Function: {}", func.sig.ident);
                }
                syn::Item::Use(_use_item) => {
                    println!("  - Use item");
                }
                syn::Item::Static(static_item) => {
                    println!("  - Static: {}", static_item.ident);
                }
                _ => {
                    println!("  - Other item: {:?}", std::mem::discriminant(item));
                }
            }
        }
    } else {
        println!("✅ Found main function(s):");
        for symbol in main_symbols {
            println!("  - {}: {}", symbol.name, symbol.symbol_type);
        }
    }
}
