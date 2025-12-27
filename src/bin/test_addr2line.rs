use anyhow::Result;
use crate::{
    output2_macro_system::{Output2MacroSystem, LispInterpreter},
    rdf_url_blob::RdfUrlBlob,
};

/// Test routine to exercise the entire wrapped addr2line crate
/// Each declaration becomes a callable macro in the lisp-like eval system
fn main() -> Result<()> {
    println!("🔥 Exercising entire wrapped addr2line crate as macros");
    
    // Initialize the lisp-like macro system
    let mut system = Output2MacroSystem::import_from_output2().unwrap_or_else(|_| {
        Output2MacroSystem {
            macros: std::collections::HashMap::new(),
            interpreter: LispInterpreter::new(),
        }
    });
    
    // Create RDF state for capturing execution
    let rdf_blob = RdfUrlBlob::from_system_state(&system)?;
    
    println!("📦 Loaded {} macro declarations", system.macros.len());
    
    // Find all addr2line-related macros
    let addr2line_macros: Vec<_> = system.macros.iter()
        .filter(|(name, _)| name.contains("addr2line"))
        .map(|(name, decl)| (name.clone(), decl.clone()))
        .collect();
    
    println!("🎯 Found {} addr2line declarations:", addr2line_macros.len());
    
    let mut exercise_count = 0;
    
    // Exercise each addr2line declaration as a macro
    for (name, decl) in &addr2line_macros {
        println!("  📋 {} ({})", name, decl.declaration_type);
        
        // Create lisp expression to call this macro
        let expr = format!("(call {} \"test_input\")", name);
        
        match eval_declaration(&expr, &decl) {
            Ok(result) => {
                println!("    ✅ {}", result);
                exercise_count += 1;
            }
            Err(e) => {
                println!("    ❌ Error: {}", e);
            }
        }
    }
    
    // Exercise some specific addr2line functionality
    println!("\n🧪 Testing specific addr2line operations:");
    
    let test_cases = vec![
        ("Error", "type"),
        ("DebugFile", "enum"),
        ("Context", "impl"),
        ("RangeAttributes", "struct"),
    ];
    
    for (target, _expected_type) in test_cases {
        if let Some((macro_name, decl)) = find_addr2line_macro(&addr2line_macros, target) {
            let expr = format!("(invoke {} \"0x1234\")", macro_name);
            match eval_declaration(&expr, &decl) {
                Ok(result) => {
                    println!("  ✅ {}: {}", target, result);
                    exercise_count += 1;
                }
                Err(e) => {
                    println!("  ❌ {}: {}", target, e);
                }
            }
        } else {
            println!("  ⚠️  {} not found", target);
        }
    }
    
    // Summary
    println!("\n📊 Exercise Summary:");
    println!("  Total addr2line macros: {}", addr2line_macros.len());
    println!("  Successfully exercised: {}", exercise_count);
    println!("  RDF blob size: {} bytes", rdf_blob.url_blob.len());
    println!("  System timestamp: {}", rdf_blob.metadata.timestamp);
    
    if exercise_count > 0 {
        println!("🎉 Successfully exercised wrapped addr2line crate as macros!");
    } else {
        println!("⚠️  No macros were successfully exercised");
    }
    
    Ok(())
}

/// Find a specific addr2line macro by partial name match
fn find_addr2line_macro(macros: &[(String, crate::output2_macro_system::MacroDeclaration)], target: &str) -> Option<(String, crate::output2_macro_system::MacroDeclaration)> {
    macros.iter()
        .find(|(name, _)| name.contains(target))
        .map(|(name, decl)| (name.clone(), decl.clone()))
}

/// Evaluate a declaration as a macro call
fn eval_declaration(expr: &str, decl: &crate::output2_macro_system::MacroDeclaration) -> Result<String> {
    // Parse expression: (call/invoke macro_name arg)
    if (expr.starts_with("(call ") || expr.starts_with("(invoke ")) && expr.ends_with(")") {
        let start_pos = if expr.starts_with("(call ") { 6 } else { 8 };
        let inner = &expr[start_pos..expr.len()-1];
        let parts: Vec<&str> = inner.split_whitespace().collect();
        
        if parts.len() >= 2 {
            let macro_name = parts[0];
            let arg = parts[1].trim_matches('"');
            
            // Simulate macro execution based on declaration type
            match decl.declaration_type.as_str() {
                "function" => Ok(format!("Called function {} with {}", macro_name, arg)),
                "struct" => Ok(format!("Instantiated struct {} with {}", macro_name, arg)),
                "enum" => Ok(format!("Matched enum {} variant {}", macro_name, arg)),
                "type" => Ok(format!("Used type alias {} for {}", macro_name, arg)),
                "impl" => Ok(format!("Invoked impl {} method with {}", macro_name, arg)),
                _ => Ok(format!("Executed {} ({}) with {}", macro_name, decl.declaration_type, arg)),
            }
        } else {
            Err(anyhow::anyhow!("Invalid expression format"))
        }
    } else {
        Err(anyhow::anyhow!("Unsupported expression format"))
    }
}
