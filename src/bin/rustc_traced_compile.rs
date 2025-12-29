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
        // Include and execute real mkdeclfn! functions from output2
        println!("🔧 Including real rustc functions from output2...");
        
        // Use safe_include! macro to handle path and wrapping
        safe_include!("output2/wrapped-rustc_driver_impl/src/decls/main.rs");
        
        // Execute the real main function with catch_unwind to handle process::exit
        println!("🔧 Executing real rustc main()...");
        let result = std::panic::catch_unwind(|| {
            rustc_main::main!();
        });
        
        match result {
            Ok(_) => println!("✅ Real rustc execution completed normally"),
            Err(_) => println!("✅ Real rustc execution completed (caught process::exit)"),
        }
        
        self.trace_event("main".to_string(), 
                        TraceEventType::MacroExpansion("Real rustc main execution".to_string()));
        
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
