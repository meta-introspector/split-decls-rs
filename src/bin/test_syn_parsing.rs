use syn::visit::Visit;
use std::collections::HashSet;

struct TestVisitor {
    found_tokens: HashSet<String>,
}

impl<'ast> Visit<'ast> for TestVisitor {
    fn visit_ident(&mut self, node: &'ast syn::Ident) {
        self.found_tokens.insert(node.to_string());
        syn::visit::visit_ident(self, node);
    }
    
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        if let Some(segment) = node.path.segments.last() {
            self.found_tokens.insert(segment.ident.to_string());
        }
        syn::visit::visit_macro(self, node);
    }
}

fn main() {
    println!("🧪 Testing syn parsing issues with rustc code patterns");
    
    // Test 1: Macro invocation
    test_macro_invocation();
    
    // Test 2: Conditional compilation
    test_cfg_conditional();
    
    // Test 3: Actual rustc files
    test_actual_files();
}

fn test_macro_invocation() {
    println!("\n🔍 Test 1: Macro invocation parsing");
    let code = "TimePassesCallbacks!();";
    
    if let Ok(parsed) = syn::parse_file(code) {
        let mut visitor = TestVisitor { found_tokens: HashSet::new() };
        visitor.visit_file(&parsed);
        
        let found = visitor.found_tokens.contains("TimePassesCallbacks");
        println!("  TimePassesCallbacks macro: {}", if found { "✅ Found" } else { "❌ Missing" });
        println!("  All tokens: {:?}", visitor.found_tokens);
    }
}

fn test_cfg_conditional() {
    println!("\n🔍 Test 2: Conditional compilation");
    let code = r#"
        #[cfg(unix)]
        pub fn get_resident_set_size() -> Option<usize> {
            None
        }
    "#;
    
    if let Ok(parsed) = syn::parse_file(code) {
        let mut visitor = TestVisitor { found_tokens: HashSet::new() };
        visitor.visit_file(&parsed);
        
        let found = visitor.found_tokens.contains("get_resident_set_size");
        println!("  get_resident_set_size in cfg: {}", if found { "✅ Found" } else { "❌ Missing" });
        println!("  All tokens: {:?}", visitor.found_tokens);
    }
}

fn test_actual_files() {
    println!("\n🔍 Test 3: Actual rustc files");
    
    // Test macro_383.rs
    let macro_file = "output2/wrapped-split_rustc_data_structures/src/decls/macro_383.rs";
    if let Ok(content) = std::fs::read_to_string(macro_file) {
        if let Ok(parsed) = syn::parse_file(&content) {
            let mut visitor = TestVisitor { found_tokens: HashSet::new() };
            visitor.visit_file(&parsed);
            
            println!("  📁 macro_383.rs: {} tokens found", visitor.found_tokens.len());
            
            let checks = ["get_resident_set_size", "cfg_select"];
            for check in &checks {
                let found = visitor.found_tokens.contains(*check);
                println!("    {}: {}", check, if found { "✅" } else { "❌" });
            }
        }
    }
    
    // Test main.rs
    let main_file = "output2/wrapped-rustc_driver_impl/src/decls/main.rs";
    if let Ok(content) = std::fs::read_to_string(main_file) {
        if let Ok(parsed) = syn::parse_file(&content) {
            let mut visitor = TestVisitor { found_tokens: HashSet::new() };
            visitor.visit_file(&parsed);
            
            println!("  📁 main.rs: {} tokens found", visitor.found_tokens.len());
            
            let checks = ["TimePassesCallbacks", "main", "get_resident_set_size"];
            for check in &checks {
                let found = visitor.found_tokens.contains(*check);
                println!("    {}: {}", check, if found { "✅" } else { "❌" });
            }
        }
    }
}
