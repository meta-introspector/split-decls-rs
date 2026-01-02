use syn::parse_file;
use split_decls_genesis::build_lib::*;
use std::fs;

macro_rules! runbuild {
    ($source_path:expr) => {
        {
            println!("🔧 Running build process on: {}", $source_path);
            
            // Read original source
            let original = fs::read_to_string($source_path).expect("Failed to read source file");
            println!("1️⃣ Original source loaded ({} bytes)", original.len());
            
            // Test original parsing
            match parse_file(&original) {
                Ok(_) => println!("✅ Original parses fine"),
                Err(e) => {
                    println!("❌ Original source broken: {}", e);
                    return;
                }
            }
            
            // Apply transformations step by step with auditing
            println!("\n2️⃣ Running audited process_content...");
            match process_content_with_audit(&original) {
                Ok((result, audit)) => {
                    println!("✅ Audited process completed ({} bytes)", result.len());
                    
                    // Print audit results
                    println!("\n📊 Transformation Audit:");
                    for (i, step) in audit.steps.iter().enumerate() {
                        let status = if step.success { "✅" } else { "❌" };
                        println!("  {}. {} {} {}", 
                            i+1, status, step.name,
                            if let Some(ref error) = step.error {
                                format!("- ERROR: {}", error)
                            } else {
                                "- OK".to_string()
                            }
                        );
                    }
                    
                    if !audit.bisection_log.is_empty() {
                        println!("\n🔍 Bisection Log:");
                        for log_entry in &audit.bisection_log {
                            println!("  {}", log_entry);
                        }
                    }
                    
                    // Test final result parsing
                    match parse_file(&result) {
                        Ok(_) => println!("\n✅ Final result parses fine"),
                        Err(e) => println!("\n❌ Final result broken: {}", e),
                    }
                    
                    println!("\n📈 Summary: {} successful transformations, final success: {}", 
                        audit.steps.iter().filter(|s| s.success).count(),
                        audit.final_success
                    );
                }
                Err(e) => println!("❌ Audited process failed: {}", e),
            }
        }
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }
    
    let source_path = &args[1];
    runbuild!(source_path);
}
