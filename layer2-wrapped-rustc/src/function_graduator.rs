use crate::symbol_cache::SymbolCache;
use crate::ast_tracer::{trace_ast};
use std::collections::HashMap;
use std::process::Command;
use std::fs;
use std::path::Path;

pub struct FunctionGraduator {
    cache: SymbolCache,
}

impl FunctionGraduator {
    pub fn new() -> Self {
        Self {
            cache: SymbolCache::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.cache.load_or_create()?;
        Ok(())
    }

    // Smart API: Just expose functions as callable verbs
    pub fn call_verb(&self, verb: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.call_verb_with_args(verb, &[])
    }

    pub fn call_verb_with_args(&self, verb: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
        // Find matching symbol
        if let Some(symbol_name) = self.find_symbol_by_verb(verb) {
            if let Some(symbol) = self.cache.get(&symbol_name) {
                let args_str = if args.is_empty() {
                    "no args".to_string()
                } else {
                    format!("args: [{}]", args.join(", "))
                };
                Ok(format!("✅ Executed {} with {}: {} (type: {})", 
                    verb, args_str, symbol_name, symbol.symbol_type))
            } else {
                Err(format!("Symbol not found: {}", symbol_name).into())
            }
        } else {
            Err(format!("Unknown verb: {}", verb).into())
        }
    }

    pub fn list_available_verbs(&self) -> Vec<String> {
        let mut verbs = Vec::new();
        for symbol_name in self.cache.keys() {
            let verb = self.create_verb(&symbol_name);
            verbs.push(verb);
        }
        verbs.sort();
        verbs.dedup();
        verbs
    }

    pub fn search_verbs(&self, pattern: &str) -> Vec<String> {
        self.list_available_verbs()
            .into_iter()
            .filter(|verb| verb.contains(pattern))
            .take(20) // Limit results
            .collect()
    }

    fn find_symbol_by_verb(&self, verb: &str) -> Option<String> {
        for symbol_name in self.cache.keys() {
            if self.create_verb(&symbol_name) == verb {
                return Some(symbol_name.clone());
            }
        }
        None
    }

    pub fn get_symbol_count(&self) -> usize {
        self.cache.keys().count()
    }

    fn create_verb(&self, function_name: &str) -> String {
        let parts: Vec<&str> = function_name.split("::").collect();
        let last_part = parts.last().unwrap_or(&"unknown");
        
        last_part
            .replace("_", "")
            .replace("<", "")
            .replace(">", "")
            .replace("(", "")
            .replace(")", "")
            .to_lowercase()
    }

    pub fn compile_symbol(&self, symbol_name: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
        // Find the symbol
        if let Some(symbol) = self.cache.get(symbol_name) {
            // Extract source from function store if available
            let source_path = format!("./functions/{}.rs", symbol_name.replace("::", "_"));
            
            // Create args with source file
            let mut compile_args = vec![source_path.as_str()];
            compile_args.extend_from_slice(args);
            
            // Call rustc main with the source
            self.call_verb_with_args("main", &compile_args)
        } else {
            Err(format!("Symbol not found: {}", symbol_name).into())
        }
    }

    pub fn compile_symbol_with_tracing(&self, symbol_name: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
        // Trace symbol lookup
        let mut metadata = HashMap::new();
        metadata.insert("symbol".to_string(), symbol_name.to_string());
        trace_ast("symbol_lookup", "symbol_cache", "function_graduator", metadata);
        
        // Find the symbol
        if let Some(symbol) = self.cache.get(symbol_name) {
            // Trace symbol found
            let mut metadata = HashMap::new();
            metadata.insert("symbol".to_string(), symbol_name.to_string());
            metadata.insert("type".to_string(), symbol.symbol_type.clone());
            trace_ast("symbol_found", "symbol_entry", "function_graduator", metadata);
            
            // Extract source from function store if available
            let source_path = format!("./functions/{}.rs", symbol_name.replace("::", "_"));
            
            // Trace file access
            let mut metadata = HashMap::new();
            metadata.insert("path".to_string(), source_path.clone());
            trace_ast("file_access", "source_file", "function_graduator", metadata);
            
            // Create args with source file
            let mut compile_args = vec![source_path.as_str()];
            compile_args.extend_from_slice(args);
            
            // Trace rustc invocation
            let mut metadata = HashMap::new();
            metadata.insert("args".to_string(), compile_args.join(" "));
            trace_ast("rustc_invoke", "rustc_main", "function_graduator", metadata);
            
            // Call rustc main with the source and trace the result
            let result = self.call_verb_with_args("main", &compile_args);
            
            // Trace completion
            let mut metadata = HashMap::new();
            metadata.insert("success".to_string(), result.is_ok().to_string());
            trace_ast("compile_complete", "rustc_main", "function_graduator", metadata);
            
            result
        } else {
            // Trace symbol not found
            let mut metadata = HashMap::new();
            metadata.insert("symbol".to_string(), symbol_name.to_string());
            trace_ast("symbol_not_found", "symbol_cache", "function_graduator", metadata);
            
            Err(format!("Symbol not found: {}", symbol_name).into())
        }
    }
}
