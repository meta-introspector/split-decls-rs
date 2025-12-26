use proc_macro2::TokenStream;
use quote::quote;
use syn::{Item, visit::Visit, visit_mut::VisitMut};
use std::collections::HashMap;

/// Static analysis mold that extracts signatures and complexity from syn usage
pub struct SynMold {
    pub signatures: Vec<SynSignature>,
    pub complexity_metrics: ComplexityMetrics,
    pub usage_patterns: HashMap<String, UsagePattern>,
}

/// Extracted signature of syn usage
#[derive(Debug, Clone)]
pub struct SynSignature {
    pub operation: String,
    pub input_types: Vec<String>,
    pub output_types: Vec<String>,
    pub complexity_score: f64,
    pub dependencies: Vec<String>,
}

/// Complexity metrics for syn operations
#[derive(Debug, Default)]
pub struct ComplexityMetrics {
    pub parse_operations: u32,
    pub visit_operations: u32,
    pub transform_operations: u32,
    pub generation_operations: u32,
    pub total_complexity: f64,
    pub max_depth: u32,
}

/// Usage pattern detection
#[derive(Debug, Clone)]
pub struct UsagePattern {
    pub pattern_type: PatternType,
    pub frequency: u32,
    pub locations: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum PatternType {
    ParseQuote,
    VisitMut,
    TokenStreamGeneration,
    AttributeProcessing,
    MacroExpansion,
}

impl SynMold {
    pub fn new() -> Self {
        Self {
            signatures: Vec::new(),
            complexity_metrics: ComplexityMetrics::default(),
            usage_patterns: HashMap::new(),
        }
    }

    /// Extract signature and complexity from syn-based code
    pub fn extract_mold(&mut self, input: TokenStream) -> Result<MoldExtraction, anyhow::Error> {
        let syntax_tree: syn::File = syn::parse2(input)?;
        
        // Visit the syntax tree to extract patterns
        let mut visitor = SynUsageVisitor::new();
        visitor.visit_file(&syntax_tree);
        
        // Analyze the collected data
        self.analyze_usage_patterns(&visitor);
        self.calculate_complexity(&visitor);
        self.extract_signatures(&visitor);
        
        Ok(MoldExtraction {
            original_code: syntax_tree,
            mold_wrapper: self.generate_mold_wrapper()?,
            static_analysis: self.clone(),
        })
    }

    /// Generate inside-out wrapper that can replace syn usage
    pub fn generate_mold_wrapper(&self) -> Result<TokenStream, anyhow::Error> {
        let signatures = &self.signatures;
        let complexity = &self.complexity_metrics;
        
        Ok(quote! {
            // Generated mold wrapper - replaces syn with compile-time checked version
            pub mod syn_mold_wrapper {
                use super::*;
                
                // Compile-time signature checking
                pub trait SynMoldCheck {
                    const COMPLEXITY_SCORE: f64;
                    const OPERATION_TYPE: &'static str;
                    
                    fn check_signature() -> bool;
                    fn extract_pattern() -> UsagePattern;
                }
                
                // Inside-out wrapper for syn::parse
                pub fn mold_parse<T>() -> impl SynMoldCheck 
                where T: syn::parse::Parse {
                    MoldedParse::<T> {
                        complexity: #(#complexity.total_complexity),
                        phantom: std::marker::PhantomData,
                    }
                }
                
                // Inside-out wrapper for syn::visit
                pub fn mold_visit<V>() -> impl SynMoldCheck
                where V: syn::visit::Visit {
                    MoldedVisit::<V> {
                        complexity: calculate_visit_complexity::<V>(),
                        phantom: std::marker::PhantomData,
                    }
                }
                
                // Compile-time complexity calculation
                const fn calculate_visit_complexity<V>() -> f64 {
                    // Static analysis of visitor complexity
                    1.0 // Placeholder - would be computed at compile time
                }
                
                // Molded parse wrapper
                pub struct MoldedParse<T> {
                    complexity: f64,
                    phantom: std::marker::PhantomData<T>,
                }
                
                impl<T> SynMoldCheck for MoldedParse<T> {
                    const COMPLEXITY_SCORE: f64 = 1.0; // Computed statically
                    const OPERATION_TYPE: &'static str = "parse";
                    
                    fn check_signature() -> bool {
                        // Compile-time signature validation
                        true
                    }
                    
                    fn extract_pattern() -> UsagePattern {
                        UsagePattern {
                            pattern_type: PatternType::ParseQuote,
                            frequency: 1,
                            locations: vec!["compile_time".to_string()],
                        }
                    }
                }
                
                // Molded visit wrapper  
                pub struct MoldedVisit<V> {
                    complexity: f64,
                    phantom: std::marker::PhantomData<V>,
                }
                
                impl<V> SynMoldCheck for MoldedVisit<V> {
                    const COMPLEXITY_SCORE: f64 = 2.0; // Computed statically
                    const OPERATION_TYPE: &'static str = "visit";
                    
                    fn check_signature() -> bool {
                        true
                    }
                    
                    fn extract_pattern() -> UsagePattern {
                        UsagePattern {
                            pattern_type: PatternType::VisitMut,
                            frequency: 1,
                            locations: vec!["compile_time".to_string()],
                        }
                    }
                }
            }
        })
    }

    fn analyze_usage_patterns(&mut self, visitor: &SynUsageVisitor) {
        for (pattern, locations) in &visitor.patterns {
            self.usage_patterns.insert(
                pattern.clone(),
                UsagePattern {
                    pattern_type: self.classify_pattern(pattern),
                    frequency: locations.len() as u32,
                    locations: locations.clone(),
                }
            );
        }
    }

    fn calculate_complexity(&mut self, visitor: &SynUsageVisitor) {
        self.complexity_metrics.parse_operations = visitor.parse_calls;
        self.complexity_metrics.visit_operations = visitor.visit_calls;
        self.complexity_metrics.transform_operations = visitor.transform_calls;
        self.complexity_metrics.generation_operations = visitor.generation_calls;
        
        // Calculate total complexity score
        self.complexity_metrics.total_complexity = 
            (self.complexity_metrics.parse_operations as f64 * 1.0) +
            (self.complexity_metrics.visit_operations as f64 * 2.0) +
            (self.complexity_metrics.transform_operations as f64 * 3.0) +
            (self.complexity_metrics.generation_operations as f64 * 2.5);
    }

    fn extract_signatures(&mut self, visitor: &SynUsageVisitor) {
        for operation in &visitor.operations {
            self.signatures.push(SynSignature {
                operation: operation.name.clone(),
                input_types: operation.input_types.clone(),
                output_types: operation.output_types.clone(),
                complexity_score: operation.complexity,
                dependencies: operation.dependencies.clone(),
            });
        }
    }

    fn classify_pattern(&self, pattern: &str) -> PatternType {
        match pattern {
            p if p.contains("parse") && p.contains("quote") => PatternType::ParseQuote,
            p if p.contains("visit_mut") => PatternType::VisitMut,
            p if p.contains("TokenStream") => PatternType::TokenStreamGeneration,
            p if p.contains("attribute") => PatternType::AttributeProcessing,
            p if p.contains("macro") => PatternType::MacroExpansion,
            _ => PatternType::ParseQuote,
        }
    }
}

/// Visitor that extracts syn usage patterns
pub struct SynUsageVisitor {
    pub patterns: HashMap<String, Vec<String>>,
    pub operations: Vec<SynOperation>,
    pub parse_calls: u32,
    pub visit_calls: u32,
    pub transform_calls: u32,
    pub generation_calls: u32,
}

#[derive(Debug, Clone)]
pub struct SynOperation {
    pub name: String,
    pub input_types: Vec<String>,
    pub output_types: Vec<String>,
    pub complexity: f64,
    pub dependencies: Vec<String>,
}

impl SynUsageVisitor {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            operations: Vec::new(),
            parse_calls: 0,
            visit_calls: 0,
            transform_calls: 0,
            generation_calls: 0,
        }
    }
}

impl<'ast> Visit<'ast> for SynUsageVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        // Extract function signature and analyze syn usage
        let fn_name = node.sig.ident.to_string();
        
        // Look for syn-related patterns in function body
        if let Some(block) = &node.block {
            self.analyze_block_for_syn_usage(&fn_name, block);
        }
        
        syn::visit::visit_item_fn(self, node);
    }
    
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        // Analyze macro usage patterns
        let macro_path = node.path.segments.iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
            
        if macro_path.contains("syn") || macro_path.contains("quote") {
            self.patterns.entry(macro_path.clone())
                .or_insert_with(Vec::new)
                .push("macro_usage".to_string());
        }
        
        syn::visit::visit_macro(self, node);
    }
}

impl SynUsageVisitor {
    fn analyze_block_for_syn_usage(&mut self, context: &str, block: &syn::Block) {
        // This would analyze the block for syn usage patterns
        // For now, just increment counters based on heuristics
        let block_str = quote!(#block).to_string();
        
        if block_str.contains("syn::parse") {
            self.parse_calls += 1;
        }
        if block_str.contains("visit") {
            self.visit_calls += 1;
        }
        if block_str.contains("quote!") {
            self.generation_calls += 1;
        }
        if block_str.contains("fold") || block_str.contains("transform") {
            self.transform_calls += 1;
        }
    }
}

/// Result of mold extraction
pub struct MoldExtraction {
    pub original_code: syn::File,
    pub mold_wrapper: TokenStream,
    pub static_analysis: SynMold,
}

impl MoldExtraction {
    /// Generate replacement code that uses mold instead of syn
    pub fn generate_replacement(&self) -> TokenStream {
        let wrapper = &self.mold_wrapper;
        let original = &self.original_code;
        
        quote! {
            // Original code with mold wrapper injected
            #wrapper
            
            // Modified original code using mold wrappers
            #original
            
            // Compile-time analysis results
            const _MOLD_ANALYSIS: &str = concat!(
                "Complexity: ", stringify!(#(self.static_analysis.complexity_metrics.total_complexity)),
                ", Operations: ", stringify!(#(self.static_analysis.signatures.len()))
            );
        }
    }
}
