use std::collections::HashMap;
use std::fs;
use serde_json;
use syn::visit::Visit;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ComprehensiveIndex {
    total_files_processed: usize,
    total_tokens_found: usize,
    token_classifications: HashMap<String, TokenClassification>,
    conditional_paths: HashMap<String, Vec<String>>,
    file_coverage: HashMap<String, FileCoverage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClassification {
    token: String,
    classification: String,
    occurrences: usize,
    file_locations: Vec<String>,
    contexts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileCoverage {
    path: String,
    parsed_successfully: bool,
    tokens_extracted: usize,
    parse_error: Option<String>,
}

fn main() {
    println!("🔍 Building comprehensive syn index with full token classification...");
    
    let cache_data = fs::read_to_string("dependency_cache.json").expect("Failed to read dependency_cache.json");
    let cache: serde_json::Value = serde_json::from_str(&cache_data).expect("Failed to parse JSON");
    
    let mut index = ComprehensiveIndex {
        total_files_processed: 0,
        total_tokens_found: 0,
        token_classifications: HashMap::new(),
        conditional_paths: HashMap::new(),
        file_coverage: HashMap::new(),
    };
    
    if let Some(files) = cache.get("files") {
        if let Some(files_obj) = files.as_object() {
            for (file_path, _deps) in files_obj {
                index.total_files_processed += 1;
                println!("📝 [{}/{}] Processing: {}", index.total_files_processed, files_obj.len(), file_path);
                
                process_file_comprehensively(file_path, &mut index);
                
                if index.total_files_processed % 100 == 0 {
                    println!("📊 Progress: {} files, {} tokens", index.total_files_processed, index.total_tokens_found);
                }
            }
        }
    }
    
    println!("📊 Final stats: {} files, {} unique tokens", index.total_files_processed, index.token_classifications.len());
    
    // Save comprehensive index
    let index_json = serde_json::to_string_pretty(&index).expect("Failed to serialize");
    fs::write("comprehensive_syn_index.json", index_json).expect("Failed to write index");
    
    // Save just the token->file mapping for fast lookups
    let mut fast_lookup: HashMap<String, String> = HashMap::new();
    for (token, classification) in &index.token_classifications {
        if let Some(first_file) = classification.file_locations.first() {
            fast_lookup.insert(token.clone(), first_file.clone());
        }
    }
    
    let lookup_json = serde_json::to_string_pretty(&fast_lookup).expect("Failed to serialize lookup");
    fs::write("fast_token_lookup.json", lookup_json).expect("Failed to write lookup");
    
    println!("✅ Saved comprehensive_syn_index.json and fast_token_lookup.json");
    
    // Test lookups on our missing terms
    test_comprehensive_lookups(&index);
}

fn process_file_comprehensively(file_path: &str, index: &mut ComprehensiveIndex) {
    let mut coverage = FileCoverage {
        path: file_path.to_string(),
        parsed_successfully: false,
        tokens_extracted: 0,
        parse_error: None,
    };
    
    match fs::read_to_string(file_path) {
        Ok(content) => {
            match syn::parse_file(&content) {
                Ok(parsed) => {
                    coverage.parsed_successfully = true;
                    let mut visitor = ComprehensiveVisitor {
                        file_path: file_path.to_string(),
                        tokens_found: 0,
                        index,
                    };
                    visitor.visit_file(&parsed);
                    coverage.tokens_extracted = visitor.tokens_found;
                    index.total_tokens_found += visitor.tokens_found;
                }
                Err(e) => {
                    coverage.parse_error = Some(e.to_string());
                }
            }
        }
        Err(e) => {
            coverage.parse_error = Some(format!("File read error: {}", e));
        }
    }
    
    index.file_coverage.insert(file_path.to_string(), coverage);
}

struct ComprehensiveVisitor<'a> {
    file_path: String,
    tokens_found: usize,
    index: &'a mut ComprehensiveIndex,
}

impl<'a> ComprehensiveVisitor<'a> {
    fn record_token(&mut self, token: &str, classification: &str, context: &str) {
        self.tokens_found += 1;
        
        let entry = self.index.token_classifications.entry(token.to_string()).or_insert(TokenClassification {
            token: token.to_string(),
            classification: classification.to_string(),
            occurrences: 0,
            file_locations: Vec::new(),
            contexts: Vec::new(),
        });
        
        entry.occurrences += 1;
        if !entry.file_locations.contains(&self.file_path) {
            entry.file_locations.push(self.file_path.clone());
        }
        if !entry.contexts.contains(&context.to_string()) {
            entry.contexts.push(context.to_string());
        }
    }
}

impl<'a, 'ast> Visit<'ast> for ComprehensiveVisitor<'a> {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.record_token(&node.sig.ident.to_string(), "function", "item_fn");
        syn::visit::visit_item_fn(self, node);
    }
    
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.record_token(&node.ident.to_string(), "struct", "item_struct");
        syn::visit::visit_item_struct(self, node);
    }
    
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        self.record_token(&node.ident.to_string(), "enum", "item_enum");
        syn::visit::visit_item_enum(self, node);
    }
    
    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.record_token(&node.ident.to_string(), "trait", "item_trait");
        syn::visit::visit_item_trait(self, node);
    }
    
    fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {
        self.record_token(&node.ident.to_string(), "const", "item_const");
        syn::visit::visit_item_const(self, node);
    }
    
    fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {
        self.record_token(&node.ident.to_string(), "static", "item_static");
        syn::visit::visit_item_static(self, node);
    }
    
    fn visit_item_type(&mut self, node: &'ast syn::ItemType) {
        self.record_token(&node.ident.to_string(), "type_alias", "item_type");
        syn::visit::visit_item_type(self, node);
    }
    
    fn visit_item_macro(&mut self, node: &'ast syn::ItemMacro) {
        if let Some(ident) = &node.ident {
            self.record_token(&ident.to_string(), "macro", "item_macro");
        }
        syn::visit::visit_item_macro(self, node);
    }
    
    fn visit_ident(&mut self, node: &'ast syn::Ident) {
        self.record_token(&node.to_string(), "identifier", "ident");
        syn::visit::visit_ident(self, node);
    }
}

fn test_comprehensive_lookups(index: &ComprehensiveIndex) {
    println!("🔍 Testing lookups for missing rustc terms:");
    let test_terms = [
        "get_resident_set_size", "TimePassesCallbacks", "EarlyDiagCtxt", 
        "install_ice_hook", "run_compiler", "init_rustc_env_logger"
    ];
    
    for term in &test_terms {
        if let Some(classification) = index.token_classifications.get(*term) {
            println!("  ✅ {}: {} occurrences, {} files, type: {}", 
                    term, classification.occurrences, 
                    classification.file_locations.len(), 
                    classification.classification);
        } else {
            println!("  ❌ {} not found in comprehensive index", term);
        }
    }
}
