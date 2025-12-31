use split_decls_genesis::symbol_resolver::SymbolVisitor;
use syn::{visit::Visit, File};
use std::fs;

fn main() {
    let content = fs::read_to_string("test_missing_deps.rs").unwrap();
    let syntax_tree: File = syn::parse_file(&content).unwrap();
    
    let mut visitor = SymbolVisitor::new("test".to_string());
    visitor.set_file("test_missing_deps.rs".to_string());
    visitor.visit_file(&syntax_tree);
    
    println!("🔍 Testing missing dependency resolution:");
    for symbol in &visitor.symbols {
        if symbol.name == "test_missing_deps" {
            println!("📍 Function: {}", symbol.name);
            println!("🔗 Dependencies ({}):", symbol.dependencies.len());
            for dep in &symbol.dependencies {
                println!("  - {}", dep);
            }
        }
    }
}
