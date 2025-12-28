use anyhow::Result;
use bootstrap3::Bootstrap3Executor;
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Bootstrap3: Direct execution of extracted split-decls-rs functions");
        println!("Usage:");
        println!("  {} test                     - Test individual functions", args[0]);
        println!("  {} trace <crate_path>        - Execute with call tracing", args[0]);
        return Ok(());
    }
    
    let mut executor = Bootstrap3Executor::new();
    
    match args[1].as_str() {
        "test" => {
            println!("Bootstrap3: Testing extracted functions");
            let test_path = Path::new("../bootstrap2");
            executor.execute_with_tracing(test_path)?;
            println!("\nFunction tests completed!");
            println!("Audit log entries: {}", executor.get_audit_log().len());
        }
        "trace" => {
            if args.len() < 3 {
                println!("Error: Crate path required for trace command");
                return Ok(());
            }
            
            let crate_path = Path::new(&args[2]);
            println!("Bootstrap3: Call tracing on: {}", crate_path.display());
            
            executor.execute_with_tracing(crate_path)?;
            
            println!("\n🎉 CALL TRACING COMPLETED!");
            println!("📊 Audit log entries: {}", executor.get_audit_log().len());
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Use 'test' or 'workflow <crate_path>'");
        }
    }
    
    Ok(())
}
