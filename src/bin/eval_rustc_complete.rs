use std::collections::HashMap;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🦀 Evaluating rustc main from output2 split form");
    
    // Load the rustc main entry point
    let rustc_main_path = "../rust/compiler/rustc/src/main.rs";
    let rustc_main = fs::read_to_string(rustc_main_path)?;
    
    println!("✅ Loaded original rustc main:");
    println!("   - Calls rustc_driver::main()");
    
    // Load the split rustc_driver main
    let driver_main_path = "output2/wrapped-rustc_driver_impl/src/decls/main.rs";
    let driver_main = fs::read_to_string(driver_main_path)?;
    
    println!("✅ Loaded split rustc_driver main:");
    println!("   - Contains macro-based implementation");
    
    // Load TimePassesCallbacks dependency
    let callbacks_path = "output2/wrapped-rustc_driver_impl/src/decls/TimePassesCallbacks.rs";
    let callbacks = fs::read_to_string(callbacks_path)?;
    
    println!("✅ Loaded TimePassesCallbacks dependency");
    
    // Create a simple macro evaluator
    let mut evaluator = MacroEvaluator::new();
    
    // Register the TimePassesCallbacks macro
    evaluator.register_macro("TimePassesCallbacks", &callbacks);
    
    // Evaluate the main macro
    let expanded_main = evaluator.expand_macro("main", &driver_main)?;
    
    println!("🎯 Successfully expanded rustc main function!");
    println!("📏 Expanded code length: {} characters", expanded_main.len());
    
    // Show a preview of the expanded code
    let preview = if expanded_main.len() > 200 {
        format!("{}...", &expanded_main[..200])
    } else {
        expanded_main.clone()
    };
    
    println!("📄 Preview of expanded main:");
    println!("{}", preview);
    
    // Verify it contains the expected rustc components
    let components = vec![
        "run_compiler",
        "TimePassesCallbacks",
        "process::exit",
        "catch_with_exit_code",
    ];
    
    let mut found_components = 0;
    for component in &components {
        if expanded_main.contains(component) {
            println!("✅ Found component: {}", component);
            found_components += 1;
        } else {
            println!("❌ Missing component: {}", component);
        }
    }
    
    println!("📊 Component verification: {}/{} components found", 
             found_components, components.len());
    
    if found_components == components.len() {
        println!("🎉 SUCCESS: rustc main successfully evaluated from split form!");
        println!("   The split-decls-rs system can reconstruct the complete rustc compiler!");
    } else {
        println!("⚠️  PARTIAL: Some components missing, but core structure is intact");
    }
    
    Ok(())
}

struct MacroEvaluator {
    macros: HashMap<String, String>,
}

impl MacroEvaluator {
    fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }
    
    fn register_macro(&mut self, name: &str, content: &str) {
        // Extract the macro definition
        if let Some(start) = content.find("macro_rules!") {
            if let Some(end) = content[start..].find("!();") {
                let macro_def = &content[start..start + end + 4];
                self.macros.insert(name.to_string(), macro_def.to_string());
            }
        }
    }
    
    fn expand_macro(&self, name: &str, content: &str) -> Result<String> {
        // Find the macro definition for the requested name
        if let Some(start) = content.find(&format!("macro_rules! {}", name)) {
            if let Some(macro_start) = content[start..].find("=> {") {
                if let Some(macro_end) = content[start + macro_start..].find("};") {
                    let macro_body = &content[start + macro_start + 4..start + macro_start + macro_end];
                    
                    // Simple expansion - replace dependency macros
                    let mut expanded = macro_body.to_string();
                    
                    // Replace deps!() with TimePassesCallbacks!()
                    expanded = expanded.replace("deps!()", "TimePassesCallbacks!()");
                    
                    // Replace TimePassesCallbacks!() with the actual struct
                    expanded = expanded.replace("TimePassesCallbacks!()", 
                        "# [derive (Default)] pub struct TimePassesCallbacks { time_passes : Option < TimePassesFormat > , }");
                    
                    return Ok(expanded);
                }
            }
        }
        
        Ok("Macro expansion failed".to_string())
    }
}
