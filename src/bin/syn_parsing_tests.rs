// Test cases for syn parsing issues discovered in rustc dependency resolution
// Based on research.sh findings

#[cfg(test)]
mod syn_parsing_tests {
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
            // Extract macro name
            if let Some(segment) = node.path.segments.last() {
                self.found_tokens.insert(segment.ident.to_string());
            }
            syn::visit::visit_macro(self, node);
        }
    }

    #[test]
    fn test_macro_invocation_parsing() {
        // Test case 1: TimePassesCallbacks!() macro invocation
        let code = r#"
            TimePassesCallbacks!();
        "#;
        
        let parsed = syn::parse_file(code).expect("Failed to parse");
        let mut visitor = TestVisitor { found_tokens: HashSet::new() };
        visitor.visit_file(&parsed);
        
        assert!(visitor.found_tokens.contains("TimePassesCallbacks"), 
                "Should find TimePassesCallbacks macro invocation");
    }

    #[test]
    fn test_cfg_conditional_parsing() {
        // Test case 2: Function in cfg block (like get_resident_set_size)
        let code = r#"
            #[cfg(unix)]
            pub fn get_resident_set_size() -> Option<usize> {
                None
            }
        "#;
        
        let parsed = syn::parse_file(code).expect("Failed to parse");
        let mut visitor = TestVisitor { found_tokens: HashSet::new() };
        visitor.visit_file(&parsed);
        
        assert!(visitor.found_tokens.contains("get_resident_set_size"), 
                "Should find function in cfg block");
    }

    #[test]
    fn test_macro_generated_code() {
        // Test case 3: cfg_select! macro (simplified version)
        let code = r#"
            cfg_select! {
                unix => {
                    pub fn get_resident_set_size() -> Option<usize> { None }
                }
            }
        "#;
        
        let parsed = syn::parse_file(code).expect("Failed to parse");
        let mut visitor = TestVisitor { found_tokens: HashSet::new() };
        visitor.visit_file(&parsed);
        
        // This will likely fail with current syn visitor
        println!("Found tokens in cfg_select: {:?}", visitor.found_tokens);
    }

    #[test]
    fn test_struct_with_macro_call() {
        // Test case 4: Struct usage with macro (like TimePassesCallbacks::default())
        let code = r#"
            let callbacks = TimePassesCallbacks::default();
        "#;
        
        let parsed = syn::parse_file(code).expect("Failed to parse");
        let mut visitor = TestVisitor { found_tokens: HashSet::new() };
        visitor.visit_file(&parsed);
        
        assert!(visitor.found_tokens.contains("TimePassesCallbacks"), 
                "Should find struct name in method call");
    }

    #[test]
    fn test_function_call_in_expression() {
        // Test case 5: Function call (like get_resident_set_size())
        let code = r#"
            fn test() {
                let rss = get_resident_set_size();
            }
        "#;
        
        let parsed = syn::parse_file(code).expect("Failed to parse");
        let mut visitor = TestVisitor { found_tokens: HashSet::new() };
        visitor.visit_file(&parsed);
        
        assert!(visitor.found_tokens.contains("get_resident_set_size"), 
                "Should find function name in call expression");
    }
}

// Integration test with actual rustc files
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_actual_rustc_macro_file() {
        // Test with actual macro_383.rs content
        let file_path = "output2/wrapped-split_rustc_data_structures/src/decls/macro_383.rs";
        
        if let Ok(content) = std::fs::read_to_string(file_path) {
            if let Ok(parsed) = syn::parse_file(&content) {
                let mut visitor = TestVisitor { found_tokens: HashSet::new() };
                visitor.visit_file(&parsed);
                
                println!("Tokens found in macro_383.rs: {}", visitor.found_tokens.len());
                
                // Check if we find the function we know is there
                let has_get_resident = visitor.found_tokens.contains("get_resident_set_size");
                println!("Found get_resident_set_size: {}", has_get_resident);
                
                // Print first 10 tokens for debugging
                for (i, token) in visitor.found_tokens.iter().take(10).enumerate() {
                    println!("  {}: {}", i+1, token);
                }
            }
        }
    }

    #[test]
    fn test_actual_rustc_main_file() {
        // Test with actual main.rs content
        let file_path = "output2/wrapped-rustc_driver_impl/src/decls/main.rs";
        
        if let Ok(content) = std::fs::read_to_string(file_path) {
            if let Ok(parsed) = syn::parse_file(&content) {
                let mut visitor = TestVisitor { found_tokens: HashSet::new() };
                visitor.visit_file(&parsed);
                
                println!("Tokens found in main.rs: {}", visitor.found_tokens.len());
                
                // Check for known tokens
                let checks = [
                    "TimePassesCallbacks",
                    "main", 
                    "get_resident_set_size",
                    "install_ice_hook"
                ];
                
                for check in &checks {
                    let found = visitor.found_tokens.contains(*check);
                    println!("  {}: {}", check, if found { "✅" } else { "❌" });
                }
            }
        }
    }
}
