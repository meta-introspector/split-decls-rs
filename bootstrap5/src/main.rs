use anyhow::Result;
use bootstrap5::Bootstrap5Executor;
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Bootstrap5: Call tracing of extracted functions");
        println!("Usage:");
        println!("  {} trace <crate_path>   - Execute with call tracing", args[0]);
        return Ok(());
    }
    
    let mut executor = Bootstrap5Executor::new();
    
    match args[1].as_str() {
        "trace" => {
            if args.len() < 3 {
                println!("Error: Crate path required for trace command");
                return Ok(());
            }
            
            let crate_path = Path::new(&args[2]);
            println!("Bootstrap5: Call tracing execution on: {}", crate_path.display());
            
            executor.execute_with_call_tracing(crate_path)?;
            
            println!("\n🎉 CALL TRACING COMPLETED!");
            println!("📊 Audit log entries: {}", executor.get_audit_log().len());
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Use 'trace <crate_path>'");
        }
    }
    
    Ok(())
}
