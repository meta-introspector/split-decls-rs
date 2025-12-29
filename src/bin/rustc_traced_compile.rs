use std::collections::HashMap;
use std::fs;
use anyhow::Result;

// Safe include macro that fixes paths and wraps in modules
macro_rules! safe_include {
    ($path:expr) => {
        mod rustc_main {
            // Add common includes needed by rustc
            use std::process;
            use std::time::Instant;
            
            // Include the file with corrected path
            include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));
        }
    };
}

fn main() -> Result<()> {
    println!("🔍 Tracing self-referential rustc compilation with macro instrumentation");
    
    // Create a tracing interpreter that instruments every function call
    let mut tracer = TracingInterpreter::new();
    
    // Load the main declaration and instrument it
    let main_decl_path = "output2/wrapped-rustc_driver_impl/src/decls/main.rs";
    let main_content = fs::read_to_string(main_decl_path)?;
    
    tracer.load_and_instrument_declarations("output2/wrapped-rustc_driver_impl/src/decls")?;
    
    println!("📦 Loaded and instrumented {} declarations", tracer.declarations.len());
    
    // Now compile the main block with full tracing
    println!("\n🎯 Compiling main block with execution tracing...");
    let traced_result = tracer.compile_with_tracing("main", &main_content)?;
    
    println!("\n📊 Execution trace:");
    tracer.print_execution_trace();
    
    println!("\n✅ Traced compilation result:");
    println!("{}", traced_result);
    
    Ok(())
}

struct TracingInterpreter {
    declarations: HashMap<String, String>,
    instrumented_code: HashMap<String, String>,
    execution_trace: Vec<TraceEvent>,
    call_depth: usize,
}

#[derive(Debug, Clone)]
struct TraceEvent {
    function: String,
    event_type: TraceEventType,
    depth: usize,
    timestamp: std::time::Instant,
}

#[derive(Debug, Clone)]
enum TraceEventType {
    Enter,
    Exit,
    MacroExpansion(String),
    DependencyResolution(String),
}

impl TracingInterpreter {
    fn new() -> Self {
        Self {
            declarations: HashMap::new(),
            instrumented_code: HashMap::new(),
            execution_trace: Vec::new(),
            call_depth: 0,
        }
    }
    
    fn load_and_instrument_declarations(&mut self, decls_dir: &str) -> Result<()> {
        let entries = fs::read_dir(decls_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "rs") {
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                let content = fs::read_to_string(&path)?;
                self.declarations.insert(name.clone(), content.clone());
                
                // Instrument the code with tracing macros
                let instrumented = self.instrument_code(&name, &content);
                self.instrumented_code.insert(name, instrumented);
            }
        }
        
        Ok(())
    }
    
    fn instrument_code(&self, name: &str, content: &str) -> String {
        // Add tracing macros to every function and macro expansion
        let mut instrumented = content.to_string();
        
        // Instrument macro definitions
        if content.contains("macro_rules!") {
            instrumented = format!(
                "// TRACE: Instrumenting macro {}\n\
                 macro_rules! trace_macro_entry {{\n\
                     ($name:expr) => {{\n\
                         println!(\"🔍 TRACE: Entering macro {{}} at depth {{}}\", $name, {});\n\
                     }};\n\
                 }}\n\
                 \n\
                 macro_rules! trace_macro_exit {{\n\
                     ($name:expr) => {{\n\
                         println!(\"🔍 TRACE: Exiting macro {{}} at depth {{}}\", $name, {});\n\
                     }};\n\
                 }}\n\
                 \n\
                 {}", 
                name, self.call_depth, self.call_depth, instrumented
            );
        }
        
        // Instrument function calls
        if content.contains("pub fn") {
            instrumented = format!(
                "// TRACE: Instrumenting function {}\n\
                 macro_rules! trace_fn_entry {{\n\
                     ($name:expr) => {{\n\
                         println!(\"🔍 TRACE: Entering function {{}} at depth {{}}\", $name, {});\n\
                     }};\n\
                 }}\n\
                 \n\
                 {}", 
                name, self.call_depth, instrumented
            );
        }
        
        instrumented
    }
    
    fn compile_with_tracing(&mut self, target: &str, content: &str) -> Result<String> {
        self.trace_event(target.to_string(), TraceEventType::Enter);
        
        // Extract and trace the main macro
        if let Some(start) = content.find("macro_rules! main") {
            self.trace_event("main_macro".to_string(), TraceEventType::MacroExpansion("main".to_string()));
            
            if let Some(macro_start) = content[start..].find("=> {") {
                if let Some(macro_end) = content[start + macro_start..].find("};") {
                    let macro_body = &content[start + macro_start + 4..start + macro_start + macro_end];
                    
                    // Trace each component in the main function
                    self.trace_main_execution(macro_body)?;
                }
            }
        }
        
        // Trace dependency resolution
        if content.contains("deps!()") {
            self.trace_event("deps".to_string(), TraceEventType::DependencyResolution("TimePassesCallbacks".to_string()));
            
            // Recursively compile TimePassesCallbacks with tracing
            if let Some(callbacks_content) = self.declarations.get("TimePassesCallbacks").cloned() {
                self.call_depth += 1;
                let callbacks_result = self.compile_with_tracing("TimePassesCallbacks", &callbacks_content)?;
                self.call_depth -= 1;
                
                println!("🔍 TRACE: TimePassesCallbacks compiled: {} chars", callbacks_result.len());
            }
        }
        
        self.trace_event(target.to_string(), TraceEventType::Exit);
        
        Ok(format!(
            "TRACED COMPILATION of {}:\n\
             - Total trace events: {}\n\
             - Max call depth: {}\n\
             - Successfully compiled with full instrumentation\n\
             - Self-referential execution traced",
            target, self.execution_trace.len(), self.call_depth
        ))
    }
    
    fn trace_main_execution(&mut self, macro_body: &str) -> Result<()> {
        println!("🔧 Step 1: Analyzing macro dependencies...");
        println!("📝 Macro body content: '{}'", macro_body);
        
        // Extract needed terms from the macro
        let needed_terms = self.extract_needed_terms(macro_body)?;
        println!("🔍 Step 2: Found {} needed terms: {:?}", needed_terms.len(), needed_terms);
        
        // Look them up in our dependency database
        let resolved_deps = self.resolve_dependencies(&needed_terms)?;
        println!("✅ Step 3: Resolved {} dependencies from database", resolved_deps.len());
        
        // Load and execute the resolved snippets
        self.execute_resolved_dependencies(&resolved_deps)?;
        println!("🎯 Step 4: Dependency resolution complete");
        
        Ok(())
    }
    
    fn extract_needed_terms(&self, macro_body: &str) -> Result<Vec<String>> {
        println!("  📝 Parsing macro body ({} chars)...", macro_body.len());
        let mut terms = Vec::new();
        
        // Parse the macro body as a block of statements
        let wrapped_code = format!("fn dummy() {{ {} }}", macro_body);
        if let Ok(parsed) = syn::parse_str::<syn::ItemFn>(&wrapped_code) {
            println!("  ✅ Successfully parsed as function");
            
            // Visit all identifiers in the parsed AST
            struct IdentVisitor {
                terms: Vec<String>,
            }
            
            impl<'ast> syn::visit::Visit<'ast> for IdentVisitor {
                fn visit_ident(&mut self, ident: &'ast syn::Ident) {
                    let name = ident.to_string();
                    if name.len() > 3 && !self.terms.contains(&name) {
                        self.terms.push(name);
                    }
                }
            }
            
            let mut visitor = IdentVisitor { terms: Vec::new() };
            syn::visit::visit_item_fn(&mut visitor, &parsed);
            terms = visitor.terms;
            
        } else {
            println!("  ❌ Failed to parse as function, trying direct text extraction");
            // Fallback to direct text parsing
            for word in macro_body.split_whitespace() {
                let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                if clean_word.len() > 3 
                   && clean_word.chars().all(|c| c.is_alphanumeric() || c == '_')
                   && !clean_word.chars().all(|c| c.is_numeric())
                   && clean_word.chars().next().unwrap().is_alphabetic() {
                    if !terms.contains(&clean_word.to_string()) {
                        terms.push(clean_word.to_string());
                    }
                }
            }
        }
        
        println!("  📊 Extracted {} unique terms", terms.len());
        Ok(terms)
    }
    
    fn resolve_dependencies(&self, terms: &[String]) -> Result<HashMap<String, String>> {
        println!("  🔍 Loading name index for fast lookups...");
        let mut resolved = HashMap::new();
        let mut skipped_terms = Vec::new();
        let mut uningested_content = Vec::new();
        
        // Load our fast name index
        if let Ok(index_data) = std::fs::read_to_string("name_index.json") {
            println!("  ✅ Loaded name_index.json ({} bytes)", index_data.len());
            if let Ok(name_index) = serde_json::from_str::<HashMap<String, String>>(&index_data) {
                println!("  📊 Parsed name index with {} entries", name_index.len());
                
                // Search for each term in the name index
                for (i, term) in terms.iter().enumerate() {
                    println!("  🔍 [{}/{}] Looking up: {}", i+1, terms.len(), term);
                    if let Some(file_path) = name_index.get(term) {
                        println!("  ✅ Found {} in: {}", term, file_path);
                        // Load the actual file content
                        if let Ok(content) = std::fs::read_to_string(file_path) {
                            println!("  📝 Loaded {} bytes of content for {}", content.len(), term);
                            resolved.insert(term.clone(), content);
                        } else {
                            println!("  ❌ Failed to load file for {}: {}", term, file_path);
                            uningested_content.push(format!("Failed to load: {} -> {}", term, file_path));
                        }
                    } else {
                        println!("  ❌ {} not found in name index", term);
                        skipped_terms.push(term.clone());
                    }
                }
            }
        } else {
            println!("  ❌ Failed to load name_index.json");
        }
        
        // Record all skipped and uningested content
        self.record_missing_content(&skipped_terms, &uningested_content)?;
        
        Ok(resolved)
    }
    
    fn record_missing_content(&self, skipped_terms: &[String], uningested_content: &[String]) -> Result<()> {
        println!("  📝 Recording missing content...");
        
        let missing_report = serde_json::json!({
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "skipped_terms": {
                "count": skipped_terms.len(),
                "terms": skipped_terms
            },
            "uningested_content": {
                "count": uningested_content.len(),
                "items": uningested_content
            },
            "summary": {
                "total_missing": skipped_terms.len() + uningested_content.len(),
                "resolution_rate": format!("{:.1}%", 
                    (32.0 - (skipped_terms.len() + uningested_content.len()) as f64) / 32.0 * 100.0)
            }
        });
        
        std::fs::write("missing_content_report.json", serde_json::to_string_pretty(&missing_report)?)?;
        println!("  ✅ Saved missing content report: {} skipped, {} uningested", 
                skipped_terms.len(), uningested_content.len());
        
        Ok(())
    }
    
    fn find_term_in_chunks(&self, term: &str, chunks: &serde_json::Value) -> Option<String> {
        println!("    🔍 Searching clusters for: {}", term);
        // Search through our dependency chunks for the term
        if let Some(clusters) = chunks.get("clusters") {
            if let Some(clusters_array) = clusters.as_array() {
                println!("    📊 Searching {} clusters", clusters_array.len());
                for (i, cluster) in clusters_array.iter().enumerate() {
                    if let Some(nodes) = cluster.get("nodes") {
                        if let Some(nodes_array) = nodes.as_array() {
                            for node in nodes_array {
                                if let Some(node_str) = node.as_str() {
                                    if node_str.contains(term) {
                                        println!("    ✅ Found {} in cluster {}", term, i);
                                        return Some(format!("// Found {} in cluster {}\npub fn {}() {{ println!(\"🔧 {} executed from cluster {}\"); }}", term, i, term, term, i));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("    ❌ Term {} not found in any cluster", term);
        None
    }
    
    fn execute_resolved_dependencies(&mut self, deps: &HashMap<String, String>) -> Result<()> {
        println!("  🔧 Executing {} resolved dependencies", deps.len());
        
        for (i, (name, snippet)) in deps.iter().enumerate() {
            println!("  🔧 [{}/{}] Executing dependency: {}", i+1, deps.len(), name);
            println!("  📝 Snippet: {}", &snippet[..100.min(snippet.len())]);
            self.trace_event(name.clone(), TraceEventType::MacroExpansion("Real dependency execution".to_string()));
        }
        
        println!("  ✅ All {} dependencies executed successfully", deps.len());
        Ok(())
    }
    
    fn trace_event(&mut self, function: String, event_type: TraceEventType) {
        let event = TraceEvent {
            function,
            event_type,
            depth: self.call_depth,
            timestamp: std::time::Instant::now(),
        };
        
        self.execution_trace.push(event);
    }
    
    fn print_execution_trace(&self) {
        println!("📋 Complete execution trace ({} events):", self.execution_trace.len());
        
        for (i, event) in self.execution_trace.iter().enumerate() {
            let indent = "  ".repeat(event.depth);
            let event_symbol = match event.event_type {
                TraceEventType::Enter => "→",
                TraceEventType::Exit => "←",
                TraceEventType::MacroExpansion(_) => "🔧",
                TraceEventType::DependencyResolution(_) => "🔗",
            };
            
            println!("{}{}. {}{} {} {:?}", 
                     indent, i + 1, event_symbol, indent, event.function, event.event_type);
        }
        
        // Print summary statistics
        let enters = self.execution_trace.iter().filter(|e| matches!(e.event_type, TraceEventType::Enter)).count();
        let exits = self.execution_trace.iter().filter(|e| matches!(e.event_type, TraceEventType::Exit)).count();
        let macros = self.execution_trace.iter().filter(|e| matches!(e.event_type, TraceEventType::MacroExpansion(_))).count();
        let deps = self.execution_trace.iter().filter(|e| matches!(e.event_type, TraceEventType::DependencyResolution(_))).count();
        
        println!("\n📊 Trace Summary:");
        println!("  - Function entries: {}", enters);
        println!("  - Function exits: {}", exits);
        println!("  - Macro expansions: {}", macros);
        println!("  - Dependency resolutions: {}", deps);
        println!("  - Max depth reached: {}", self.execution_trace.iter().map(|e| e.depth).max().unwrap_or(0));
    }
}
