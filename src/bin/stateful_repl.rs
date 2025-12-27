use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use anyhow::Result;
use crate::{
    rdf_url_blob::RdfUrlBlob,
    output2_macro_system::{Output2MacroSystem, MacroDeclaration, LispInterpreter},
};

const STATE_FILE: &str = "repl_state.rdf";

#[derive(Debug)]
struct ReplState {
    system: Output2MacroSystem,
    variables: HashMap<String, String>,
    history: Vec<String>,
}

impl ReplState {
    fn new() -> Self {
        Self {
            system: Output2MacroSystem::import_from_output2().unwrap_or_else(|_| {
                // Fallback to empty system if output2 doesn't exist
                Output2MacroSystem {
                    macros: HashMap::new(),
                    interpreter: LispInterpreter::new(),
                }
            }),
            variables: HashMap::new(),
            history: Vec::new(),
        }
    }
    
    fn load_from_file() -> Result<Self> {
        Self::load_from_file_path(STATE_FILE)
    }
    
    fn load_from_file_path(path: &str) -> Result<Self> {
        if Path::new(path).exists() {
            let rdf_content = fs::read_to_string(path)?;
            let blob = RdfUrlBlob::from_url_blob(&rdf_content)?;
            
            // Parse RDF back to system state (simplified)
            let mut state = Self::new();
            
            // Extract variables from RDF comments
            for line in blob.rdf_content.lines() {
                if line.starts_with("# var:") {
                    if let Some((key, value)) = line[6..].split_once("=") {
                        state.variables.insert(key.trim().to_string(), value.trim().to_string());
                    }
                }
            }
            
            println!("📂 Loaded state from {}", path);
            Ok(state)
        } else {
            println!("🆕 Starting fresh REPL session");
            Ok(Self::new())
        }
    }
    
    fn load_from_url(url: &str) -> Result<Self> {
        println!("🌐 Loading state from URL: {}", url);
        
        // Convert GitHub blob URL to raw URL
        let raw_url = if url.contains("github.com") && url.contains("/blob/") {
            url.replace("github.com", "raw.githubusercontent.com").replace("/blob/", "/")
        } else {
            url.to_string()
        };
        
        // Fetch content (simplified - in production use reqwest)
        let content = std::process::Command::new("curl")
            .arg("-s")
            .arg(&raw_url)
            .output()?;
            
        if !content.status.success() {
            return Err(anyhow::anyhow!("Failed to fetch URL: {}", raw_url));
        }
        
        let code_content = String::from_utf8(content.stdout)?;
        
        // Create a macro from the fetched declaration
        let mut state = Self::new();
        let filename = url.split('/').last().unwrap_or("unknown");
        let macro_name = filename.trim_end_matches(".rs").replace("_decls_", "_");
        
        let decl = MacroDeclaration {
            name: macro_name.clone(),
            source_path: url.to_string(),
            declaration_type: "declaration".to_string(),
            content: code_content,
            wrapper: None,
        };
        
        state.system.macros.insert(macro_name.clone(), decl);
        println!("✅ Loaded declaration as macro: {}", macro_name);
        
        Ok(state)
    }
    
    fn save_to_file(&self) -> Result<()> {
        let blob = RdfUrlBlob::from_system_state(&self.system)?;
        
        // Add variables as RDF comments
        let mut enhanced_rdf = String::new();
        enhanced_rdf.push_str("# REPL State Variables\n");
        for (key, value) in &self.variables {
            enhanced_rdf.push_str(&format!("# var:{}={}\n", key, value));
        }
        enhanced_rdf.push_str("\n");
        enhanced_rdf.push_str(&blob.rdf_content);
        
        fs::write(STATE_FILE, blob.url_blob)?;
        println!("💾 State saved to {}", STATE_FILE);
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("🚀 Stateful REPL - RDF Blob State System");
    println!("Commands: set <var>=<value>, get <var>, save, load, export <type>, quit");
    println!("State file: {}\n", STATE_FILE);
    
    let mut state = ReplState::load_from_file()?;
    
    loop {
        print!("repl> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        state.history.push(input.to_string());
        
        match execute_command(&mut state, input) {
            Ok(should_continue) => {
                if !should_continue {
                    break;
                }
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
    }
    
    state.save_to_file()?;
    println!("👋 Goodbye!");
    Ok(())
}

fn execute_command(state: &mut ReplState, input: &str) -> Result<bool> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    match parts.get(0) {
        Some(&"quit") | Some(&"exit") => {
            return Ok(false);
        }
        
        Some(&"set") => {
            if parts.len() < 2 {
                println!("Usage: set <var>=<value>");
                return Ok(true);
            }
            
            let assignment = parts[1..].join(" ");
            if let Some((key, value)) = assignment.split_once('=') {
                state.variables.insert(key.trim().to_string(), value.trim().to_string());
                println!("✅ Set {} = {}", key.trim(), value.trim());
            } else {
                println!("❌ Invalid format. Use: set var=value");
            }
        }
        
        Some(&"get") => {
            if let Some(var) = parts.get(1) {
                if let Some(value) = state.variables.get(*var) {
                    println!("{} = {}", var, value);
                } else {
                    println!("❌ Variable '{}' not found", var);
                }
            } else {
                println!("📋 Variables:");
                for (key, value) in &state.variables {
                    println!("  {} = {}", key, value);
                }
            }
        }
        
        Some(&"save") => {
            state.save_to_file()?;
        }
        
        Some(&"load") => {
            if let Some(source) = parts.get(1) {
                if source.starts_with("http") {
                    // Load from URL
                    *state = ReplState::load_from_url(source)?;
                } else {
                    // Load from file
                    *state = ReplState::load_from_file_path(source)?;
                }
            } else {
                *state = ReplState::load_from_file()?;
            }
        }
        
        Some(&"export") => {
            if let Some(export_type) = parts.get(1) {
                let blob = RdfUrlBlob::from_system_state(&state.system)?;
                match blob.export(export_type) {
                    Ok(content) => println!("{}", content),
                    Err(e) => println!("❌ Export failed: {}", e),
                }
            } else {
                println!("Usage: export <macros|code|docs|workspace|rdf|url>");
            }
        }
        
        Some(&"info") => {
            let blob = RdfUrlBlob::from_system_state(&state.system)?;
            blob.print_info();
            println!("📊 REPL Stats:");
            println!("  Variables: {}", state.variables.len());
            println!("  History: {} commands", state.history.len());
        }
        
        Some(&"history") => {
            println!("📜 Command History:");
            for (i, cmd) in state.history.iter().enumerate() {
                println!("  {}: {}", i + 1, cmd);
            }
        }
        
        Some(&"clear") => {
            state.variables.clear();
            state.history.clear();
            println!("🧹 Cleared variables and history");
        }
        
        Some(&"macro") => {
            if parts.len() < 3 {
                println!("Usage: macro <name> <code>");
                return Ok(true);
            }
            
            let name = parts[1].to_string();
            let code = parts[2..].join(" ");
            
            let decl = MacroDeclaration {
                name: name.clone(),
                source_path: format!("repl:{}", name),
                declaration_type: "macro".to_string(),
                content: code.clone(),
                wrapper: Some(code),
            };
            
            state.system.macros.insert(name.clone(), decl);
            println!("✅ Added macro: {}", name);
        }
        
        Some(&"call") => {
            if let Some(macro_name) = parts.get(1) {
                if let Some(decl) = state.system.macros.get(*macro_name) {
                    println!("🎭 Calling macro: {}", macro_name);
                    println!("📄 Content:");
                    println!("{}", decl.content);
                    
                    // For demonstration, we'll show the macro content
                    // In a real implementation, you'd compile and execute it
                    if let Some(wrapper) = &decl.wrapper {
                        println!("🔧 Wrapper:");
                        println!("{}", wrapper);
                    }
                } else {
                    println!("❌ Macro '{}' not found", macro_name);
                }
            } else {
                println!("Usage: call <macro_name>");
            }
        }
        
        Some(&"help") => {
            println!("📖 Available Commands:");
            println!("  set <var>=<value>  - Set a variable");
            println!("  get [var]          - Get variable or list all");
            println!("  macro <name> <code> - Define a macro");
            println!("  call <macro_name>  - Execute a loaded macro");
            println!("  save               - Save state to file");
            println!("  load [url|file]    - Load state from file or GitHub URL");
            println!("  export <type>      - Export (macros|code|docs|workspace|rdf|url)");
            println!("  info               - Show state information");
            println!("  history            - Show command history");
            println!("  clear              - Clear variables and history");
            println!("  help               - Show this help");
            println!("  quit/exit          - Exit REPL");
        }
        
        _ => {
            println!("❓ Unknown command: {}. Type 'help' for available commands.", input);
        }
    }
    
    Ok(true)
}
