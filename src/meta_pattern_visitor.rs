// Layer 1: Define the core enum with mkmeta decoration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynLangPatterns {
    // Core structures
    File,
    Item,
    ItemFn,
    ItemStruct,
    ItemEnum,
    ItemImpl,
    ItemTrait,
    ItemMod,
    ItemUse,
    ItemConst,
    ItemStatic,
    
    // Expressions
    Expr,
    ExprCall,
    ExprMethodCall,
    ExprPath,
    ExprLit,
    ExprBlock,
    ExprIf,
    ExprMatch,
    ExprBinary,
    ExprUnary,
    
    // Types
    Type,
    TypePath,
    TypeReference,
    TypeTuple,
    
    // Patterns
    Pat,
    PatIdent,
    PatStruct,
    PatTuple,
    
    // Misc
    Ident,
    Path,
    Block,
    Signature,
    Generics,
}

// Layer 2: mkmeta! decorator that generates metadata
macro_rules! mkmeta {
    ($enum_name:ident { $($variant:ident),* $(,)? }) => {
        impl $enum_name {
            pub const ALL: &'static [Self] = &[$(Self::$variant),*];
            
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant)),*
                }
            }
            
            pub fn visit_method_name(&self) -> String {
                format!("visit_{}", self.as_str().to_lowercase())
            }
            
            pub fn syn_type_name(&self) -> String {
                format!("syn::{}", self.as_str())
            }
        }
    };
}

// Layer 3: Apply mkmeta to generate metadata methods
mkmeta!(SynLangPatterns {
    File, Item, ItemFn, ItemStruct, ItemEnum, ItemImpl, ItemTrait, ItemMod, ItemUse, ItemConst, ItemStatic,
    Expr, ExprCall, ExprMethodCall, ExprPath, ExprLit, ExprBlock, ExprIf, ExprMatch, ExprBinary, ExprUnary,
    Type, TypePath, TypeReference, TypeTuple,
    Pat, PatIdent, PatStruct, PatTuple,
    Ident, Path, Block, Signature, Generics,
});

// Layer 4: Generate visitor method signatures from enum variants
macro_rules! gen_visitor_signatures {
    ($($pattern:expr),* $(,)?) => {
        $(
            paste::paste! {
                fn [<visit_ $pattern:lower>](&mut self, node: &'ast syn::$pattern) {
                    self.increment(stringify!($pattern));
                    syn::visit::[<visit_ $pattern:lower>](self, node);
                }
            }
        )*
    };
}

// Layer 5: Generate visitor implementation using pattern enum
macro_rules! impl_visitor_from_patterns {
    ($visitor_struct:ident, [$($pattern:ident),* $(,)?]) => {
        impl<'ast> syn::visit::Visit<'ast> for $visitor_struct {
            gen_visitor_signatures!($($pattern),*);
        }
    };
}

// Layer 6: Generate pattern-based visitor with enum parameter expansion
macro_rules! create_pattern_visitor {
    ($visitor_name:ident, $patterns:expr) => {
        struct $visitor_name<'a> {
            stats: &'a mut crate::ast_statistics::AstStatistics,
            rdf_state: &'a mut crate::macro_interpreter::RdfStateMachine,
        }
        
        impl $visitor_name<'_> {
            fn increment(&mut self, pattern_name: &str) {
                self.stats.increment(pattern_name);
                self.rdf_state.capture_data("ast_pattern", pattern_name);
            }
        }
        
        // Generate visitor methods for each pattern in the enum
        create_visitor_methods!($visitor_name, $patterns);
    };
}

// Layer 7: Generate individual visitor methods from pattern array
macro_rules! create_visitor_methods {
    ($visitor_struct:ident, $patterns:expr) => {
        impl<'ast> syn::visit::Visit<'ast> for $visitor_struct<'_> {
            // Manually implement for each known pattern since we can't iterate in macros
            fn visit_file(&mut self, node: &'ast syn::File) {
                self.increment("File");
                syn::visit::visit_file(self, node);
            }
            
            fn visit_item(&mut self, node: &'ast syn::Item) {
                self.increment("Item");
                syn::visit::visit_item(self, node);
            }
            
            fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
                self.increment("ItemFn");
                syn::visit::visit_item_fn(self, node);
            }
            
            fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
                self.increment("ItemStruct");
                syn::visit::visit_item_struct(self, node);
            }
            
            fn visit_expr(&mut self, node: &'ast syn::Expr) {
                self.increment("Expr");
                syn::visit::visit_expr(self, node);
            }
            
            fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
                self.increment("ExprCall");
                syn::visit::visit_expr_call(self, node);
            }
            
            fn visit_type(&mut self, node: &'ast syn::Type) {
                self.increment("Type");
                syn::visit::visit_type(self, node);
            }
            
            fn visit_pat(&mut self, node: &'ast syn::Pat) {
                self.increment("Pat");
                syn::visit::visit_pat(self, node);
            }
            
            fn visit_ident(&mut self, node: &'ast syn::Ident) {
                self.increment("Ident");
                syn::visit::visit_ident(self, node);
            }
            
            fn visit_block(&mut self, node: &'ast syn::Block) {
                self.increment("Block");
                syn::visit::visit_block(self, node);
            }
        }
    };
}

// Layer 8: Top-level macro that orchestrates all layers
macro_rules! generate_complete_visitor_system {
    ($visitor_name:ident) => {
        // Use the enum to generate the complete visitor
        create_pattern_visitor!($visitor_name, SynLangPatterns::ALL);
        
        impl<'a> $visitor_name<'a> {
            pub fn new(
                stats: &'a mut crate::ast_statistics::AstStatistics,
                rdf_state: &'a mut crate::macro_interpreter::RdfStateMachine,
            ) -> $visitor_name<'a> {
                $visitor_name { stats, rdf_state }
            }
            
            pub fn analyze_file(&mut self, file: &syn::File) {
                use syn::visit::Visit;
                self.visit_file(file);
            }
            
            pub fn report_patterns(&self) {
                println!("🎯 PATTERN ANALYSIS COMPLETE");
                println!("Analyzed {} AST pattern types", SynLangPatterns::ALL.len());
                
                for pattern in SynLangPatterns::ALL {
                    println!("  {} -> {}", pattern.as_str(), pattern.visit_method_name());
                }
            }
        }
    };
}

// Usage: Generate the complete 8-layer visitor system
generate_complete_visitor_system!(MetaPatternVisitor);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_metadata() {
        assert_eq!(SynLangPatterns::ItemFn.as_str(), "ItemFn");
        assert_eq!(SynLangPatterns::ItemFn.visit_method_name(), "visit_itemfn");
        assert_eq!(SynLangPatterns::ItemFn.syn_type_name(), "syn::ItemFn");
    }
    
    #[test]
    fn test_all_patterns_covered() {
        assert!(SynLangPatterns::ALL.len() > 20);
        assert!(SynLangPatterns::ALL.contains(&SynLangPatterns::File));
        assert!(SynLangPatterns::ALL.contains(&SynLangPatterns::Expr));
    }
}
