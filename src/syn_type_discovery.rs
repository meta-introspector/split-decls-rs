use std::collections::HashSet;
use std::fs;
use syn::{visit::Visit, File, Item, ItemEnum, Variant};

// Self-discovering AST type extractor
#[derive(Debug, Default)]
pub struct SynTypeDiscovery {
    pub discovered_types: HashSet<String>,
    pub visit_methods: HashSet<String>,
    pub enum_variants: HashSet<String>,
}

impl SynTypeDiscovery {
    pub fn new() -> Self {
        Self::default()
    }

    // Query the actual syn crate source to discover live types
    pub fn discover_from_syn_source(&mut self) -> anyhow::Result<()> {
        // Check if we have syn source in our wrapped declarations
        let syn_paths = [
            "output2/wrapped-syn/src/decls/",
            "submodules/syn/src/", // If we have syn submodule
        ];

        for base_path in &syn_paths {
            if let Ok(entries) = fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    if entry.path().extension().map_or(false, |ext| ext == "rs") {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            self.extract_types_from_code(&content)?;
                        }
                    }
                }
            }
        }

        // Also discover from our own wrapped syn declarations
        self.discover_from_wrapped_declarations()?;
        
        Ok(())
    }

    // Extract enum types and variants from Rust code
    fn extract_types_from_code(&mut self, code: &str) -> anyhow::Result<()> {
        if let Ok(syntax_tree) = syn::parse_str::<File>(code) {
            let mut visitor = TypeExtractorVisitor { discovery: self };
            visitor.visit_file(&syntax_tree);
        }
        Ok(())
    }

    // Discover from our wrapped syn declarations
    fn discover_from_wrapped_declarations(&mut self) -> anyhow::Result<()> {
        let decl_invocation_path = "output2/wrapped-syn/src/decls/_decl_module_invocation.rs";
        if let Ok(content) = fs::read_to_string(decl_invocation_path) {
            // Extract module names from decl_module! macro
            for line in content.lines() {
                if line.trim().starts_with("wrapped_syn_decls_") {
                    let module_name = line.trim().trim_end_matches(',');
                    // Convert module name to potential type name
                    if let Some(type_name) = self.module_name_to_type(module_name) {
                        self.discovered_types.insert(type_name.clone());
                        self.visit_methods.insert(format!("visit_{}", type_name.to_lowercase()));
                    }
                }
            }
        }
        Ok(())
    }

    fn module_name_to_type(&self, module_name: &str) -> Option<String> {
        // Convert wrapped_syn_decls_module_not_found_expr -> Expr
        // Convert wrapped_syn_decls_parse -> Parse (special case)
        if module_name.contains("module_not_found_") {
            let parts: Vec<&str> = module_name.split("module_not_found_").collect();
            if parts.len() == 2 {
                let type_part = parts[1];
                // Capitalize first letter
                let mut chars = type_part.chars();
                if let Some(first) = chars.next() {
                    return Some(first.to_uppercase().collect::<String>() + &chars.collect::<String>());
                }
            }
        } else if module_name.contains("parse") {
            return Some("Parse".to_string());
        }
        None
    }

    // Generate the visitor macro based on discovered types
    pub fn generate_visitor_macro(&self) -> String {
        let mut macro_code = String::from("macro_rules! impl_discovered_visitor {\n");
        macro_code.push_str("    ($visitor_struct:ident) => {\n");
        macro_code.push_str("        impl<'ast> Visit<'ast> for $visitor_struct {\n");

        // Generate visit methods for discovered types
        for type_name in &self.discovered_types {
            let method_name = format!("visit_{}", type_name.to_lowercase());
            macro_code.push_str(&format!(
                "            fn {}(&mut self, node: &'ast syn::{}) {{\n",
                method_name, type_name
            ));
            macro_code.push_str(&format!(
                "                self.increment(\"{}\");\n",
                type_name
            ));
            macro_code.push_str(&format!(
                "                syn::visit::{}(self, node);\n",
                method_name
            ));
            macro_code.push_str("            }\n\n");
        }

        macro_code.push_str("        }\n");
        macro_code.push_str("    };\n");
        macro_code.push_str("}\n");

        macro_code
    }

    // Generate assertion tests to verify we found expected types
    pub fn generate_qa_assertions(&self) -> String {
        let expected_core_types = [
            "Item", "Expr", "Type", "Pat", "Stmt", "File", "Block", "Ident"
        ];

        let mut assertions = String::from("#[cfg(test)]\nmod qa_assertions {\n");
        assertions.push_str("    use super::*;\n\n");
        assertions.push_str("    #[test]\n");
        assertions.push_str("    fn test_discovered_core_types() {\n");
        assertions.push_str("        let discovery = SynTypeDiscovery::new();\n");

        for expected_type in &expected_core_types {
            assertions.push_str(&format!(
                "        assert!(discovery.discovered_types.contains(\"{}\"), \"Missing core type: {}\");\n",
                expected_type, expected_type
            ));
        }

        assertions.push_str("    }\n\n");
        
        assertions.push_str("    #[test]\n");
        assertions.push_str("    fn test_minimum_type_coverage() {\n");
        assertions.push_str("        let discovery = SynTypeDiscovery::new();\n");
        assertions.push_str(&format!(
            "        assert!(discovery.discovered_types.len() >= {}, \"Insufficient type coverage\");\n",
            expected_core_types.len()
        ));
        assertions.push_str("    }\n");
        assertions.push_str("}\n");

        assertions
    }

    pub fn report_discovery(&self) {
        println!("🔍 SYN TYPE DISCOVERY REPORT:");
        println!("  Discovered types: {}", self.discovered_types.len());
        println!("  Visit methods: {}", self.visit_methods.len());
        println!("  Enum variants: {}", self.enum_variants.len());
        
        println!("\n📋 DISCOVERED TYPES:");
        let mut types: Vec<_> = self.discovered_types.iter().collect();
        types.sort();
        for (i, type_name) in types.iter().enumerate() {
            if i % 5 == 0 { println!(); }
            print!("  {:15}", type_name);
        }
        println!();
    }
}

// Visitor to extract type information from syn source code
struct TypeExtractorVisitor<'a> {
    discovery: &'a mut SynTypeDiscovery,
}

impl<'ast> Visit<'ast> for TypeExtractorVisitor<'_> {
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        let enum_name = node.ident.to_string();
        
        // Look for syn AST enums (Item, Expr, Type, etc.)
        if self.is_syn_ast_enum(&enum_name) {
            self.discovery.discovered_types.insert(enum_name.clone());
            self.discovery.visit_methods.insert(format!("visit_{}", enum_name.to_lowercase()));
            
            // Extract variants
            for variant in &node.variants {
                let variant_name = variant.ident.to_string();
                self.discovery.enum_variants.insert(variant_name.clone());
                self.discovery.discovered_types.insert(variant_name.clone());
                self.discovery.visit_methods.insert(format!("visit_{}", variant_name.to_lowercase()));
            }
        }
        
        syn::visit::visit_item_enum(self, node);
    }
}

impl TypeExtractorVisitor<'_> {
    fn is_syn_ast_enum(&self, name: &str) -> bool {
        // Heuristics to identify syn AST enums
        matches!(name, "Item" | "Expr" | "Type" | "Pat" | "Stmt" | "Lit") ||
        name.starts_with("Item") ||
        name.starts_with("Expr") ||
        name.starts_with("Type") ||
        name.starts_with("Pat") ||
        name.starts_with("Lit")
    }
}
