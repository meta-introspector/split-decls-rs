use crate::syscall;
use std::collections::HashMap;
use crate::interpret_wrapped_decl;
use syn::{Item, Type, Pat, visit::Visit, File};
use serde::{Serialize, Deserialize};
use crate::macro_interpreter::RdfStateMachine;
use crate::interpret_syn_function;

// Layer 1: Generate visitor methods using procedural macros
macro_rules! impl_visitor_for_types {
    ($($method:ident: $type:ident),* $(,)?) => {
        $(
            fn $method(&mut self, node: &'ast syn::$type) {
                self.ast_stats.increment(stringify!($type));
                syn::visit::$method(self, node);
            }
        )*
    };
}

// Layer 2: Complete AST visitor implementation
struct AstVisitor<'a> {
    ast_stats: &'a mut AstStatistics,
    rdf_state: &'a mut RdfStateMachine,
}

impl<'ast> Visit<'ast> for AstVisitor<'_> {
    impl_visitor_for_types!(
        // Core structures
        visit_file: File,
        visit_item: Item,
        visit_item_fn: ItemFn,
        visit_item_struct: ItemStruct,
        visit_item_enum: ItemEnum,
        visit_item_impl: ItemImpl,
        visit_item_trait: ItemTrait,
        visit_item_mod: ItemMod,
        visit_item_use: ItemUse,
        visit_item_const: ItemConst,
        visit_item_static: ItemStatic,
        
        // Expressions
        visit_expr: Expr,
        visit_expr_call: ExprCall,
        visit_expr_method_call: ExprMethodCall,
        visit_expr_path: ExprPath,
        visit_expr_lit: ExprLit,
        visit_expr_block: ExprBlock,
        visit_expr_if: ExprIf,
        visit_expr_match: ExprMatch,
        visit_expr_binary: ExprBinary,
        visit_expr_unary: ExprUnary,
        
        // Types
        visit_type: Type,
        visit_type_path: TypePath,
        visit_type_reference: TypeReference,
        visit_type_tuple: TypeTuple,
        
        // Patterns
        visit_pat: Pat,
        visit_pat_ident: PatIdent,
        visit_pat_struct: PatStruct,
        visit_pat_tuple: PatTuple,
        
        // Misc
        visit_ident: Ident,
        visit_path: Path,
        visit_block: Block,
        visit_signature: Signature,
        visit_generics: Generics,
    );
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AstStatistics {
    pub enum_variants: HashMap<String, VariantStats>,
    pub constructors: HashMap<String, ConstructorStats>,
    pub parameters: HashMap<String, ParameterStats>,
    pub usage_patterns: HashMap<String, PatternStats>,
    pub type_manifold: TypeManifold,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VariantStats {
    pub count: u64,
    pub contexts: Vec<String>,
    pub dependencies: Vec<String>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConstructorStats {
    pub count: u64,
    pub parameter_types: Vec<String>,
    pub return_type: String,
    pub usage_contexts: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParameterStats {
    pub count: u64,
    pub type_name: String,
    pub positions: Vec<usize>,
    pub associated_functions: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PatternStats {
    pub count: u64,
    pub ast_nodes: Vec<String>,
    pub complexity_score: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TypeManifold {
    pub dimensions: [HashMap<String, f64>; 8], // 8D manifold
    pub relationships: HashMap<String, Vec<String>>,
    pub embeddings: HashMap<String, [f64; 8]>,
}

impl AstStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment(&mut self, node_type: &str) {
        let stats = self.enum_variants.entry(node_type.to_string()).or_default();
        stats.count += 1;
    }

    pub fn analyze_with_wrapped_syn(&mut self, code: &str, rdf_state: &mut RdfStateMachine) -> anyhow::Result<()> {
        // Use wrapped syn to parse the code
        let _result = interpret_syn_function!(rdf_state, "syn::parse_file", "output2/wrapped-syn/src/decls/wrapped_syn_decls_parse_file.rs");
        
        // Parse the actual code using the layered visitor
        let syntax_tree: File = syn::parse_str(code)?;
        let mut visitor = AstVisitor {
            ast_stats: self,
            rdf_state,
        };
        
        visitor.visit_file(&syntax_tree);
        
        // Update type manifold with collected statistics
        self.update_type_manifold(rdf_state);
        
        Ok(())
    }

    fn collect_item_stats(&mut self, item: &Item, rdf_state: &mut RdfStateMachine) {
        match item {
            Item::Enum(enum_item) => {
                let enum_name = enum_item.ident.to_string();
                rdf_state.capture_data("ast_type", "enum");
                rdf_state.capture_data("enum_name", &enum_name);
                
                for variant in &enum_item.variants {
                    let variant_name = variant.ident.to_string();
                    let stats = self.enum_variants.entry(variant_name.clone()).or_default();
                    stats.count += 1;
                    stats.contexts.push(enum_name.clone());
                    
                    // Analyze variant fields
                    match &variant.fields {
                        syn::Fields::Named(fields) => {
                            for field in &fields.named {
                                if let Some(ident) = &field.ident {
                                    stats.dependencies.push(ident.to_string());
                                }
                            }
                        }
                        syn::Fields::Unnamed(fields) => {
                            stats.dependencies.push(format!("tuple_{}", fields.unnamed.len()));
                        }
                        syn::Fields::Unit => {
                            stats.dependencies.push("unit".to_string());
                        }
                    }
                }
            }
            Item::Struct(struct_item) => {
                let struct_name = struct_item.ident.to_string();
                rdf_state.capture_data("ast_type", "struct");
                rdf_state.capture_data("struct_name", &struct_name);
                
                let constructor_stats = self.constructors.entry(struct_name.clone()).or_default();
                constructor_stats.count += 1;
                constructor_stats.return_type = struct_name;
                
                // Analyze struct fields
                match &struct_item.fields {
                    syn::Fields::Named(fields) => {
                        for field in &fields.named {
                            if let Some(ident) = &field.ident {
                                let type_name = self.type_to_string(&field.ty);
                                let param_stats = self.parameters.entry(ident.to_string()).or_default();
                                param_stats.count += 1;
                                param_stats.type_name = type_name;
                            }
                        }
                    }
                    _ => {}
                }
            }
            Item::Fn(fn_item) => {
                let fn_name = fn_item.sig.ident.to_string();
                rdf_state.capture_data("ast_type", "function");
                rdf_state.capture_data("function_name", &fn_name);
                
                let constructor_stats = self.constructors.entry(fn_name.clone()).or_default();
                constructor_stats.count += 1;
                
                // Analyze function parameters
                for (i, input) in fn_item.sig.inputs.iter().enumerate() {
                    match input {
                        syn::FnArg::Typed(pat_type) => {
                            let param_name = self.pat_to_string(&pat_type.pat);
                            let type_name = self.type_to_string(&pat_type.ty);
                            let param_stats = self.parameters.entry(param_name.clone()).or_default();
                            param_stats.count += 1;
                            param_stats.positions.push(i);
                            param_stats.associated_functions.push(fn_name.clone());
                            param_stats.type_name = type_name;
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                rdf_state.capture_data("ast_type", "other");
            }
        }
    }

    fn update_type_manifold(&mut self, rdf_state: &mut RdfStateMachine) {
        // Update 8D manifold dimensions using collected statistics
        let dimensions = [
            "complexity", "frequency", "dependency", "pattern",
            "context", "usage", "relationship", "embedding"
        ];
        
        for (i, dim_name) in dimensions.iter().enumerate() {
            let dim_map = &mut self.type_manifold.dimensions[i];
            
            // Calculate dimension values for each AST node type
            for (type_name, stats) in &self.enum_variants {
                let value = match dim_name {
                    &"frequency" => stats.count as f64,
                    &"complexity" => (stats.count as f64).ln().max(1.0),
                    &"dependency" => stats.dependencies.len() as f64,
                    &"pattern" => stats.patterns.len() as f64,
                    &"context" => stats.contexts.len() as f64,
                    _ => 1.0,
                };
                dim_map.insert(type_name.clone(), value);
                
                // Generate 8D embedding
                let embedding = self.type_manifold.embeddings.entry(type_name.clone()).or_insert([0.0; 8]);
                embedding[i] = value;
            }
        }
        
        rdf_state.capture_data("manifold_dimensions", "8");
        rdf_state.capture_data("ast_node_types", &self.enum_variants.len().to_string());
        rdf_state.emit_triple("ast_manifold", "rdf:type", "EightDimensionalTypeSpace");
    }

    fn type_to_string(&self, ty: &Type) -> String {
        match ty {
            Type::Path(type_path) => {
                type_path.path.segments.iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::")
            }
            _ => "unknown".to_string(),
        }
    }

    fn pat_to_string(&self, pat: &Pat) -> String {
        match pat {
            Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
            _ => "unknown".to_string(),
        }
    }

    pub fn emit_rdf_statistics(&self, rdf_state: &mut RdfStateMachine) {
        rdf_state.emit_triple("ast_statistics", "rdf:type", "StatisticalAnalysis");
        rdf_state.emit_triple("ast_statistics", "enum_count", &self.enum_variants.len().to_string());
        rdf_state.emit_triple("ast_statistics", "constructor_count", &self.constructors.len().to_string());
        rdf_state.emit_triple("ast_statistics", "parameter_count", &self.parameters.len().to_string());
        
        for (type_name, embedding) in &self.type_manifold.embeddings {
            for (i, value) in embedding.iter().enumerate() {
                rdf_state.emit_triple(type_name, &format!("manifold_dim_{}", i), &value.to_string());
            }
        }
    }
}
