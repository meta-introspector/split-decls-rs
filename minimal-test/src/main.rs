use anyhow::Result;

// Include generated tracing macros
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn main() -> Result<()> {
    println!("🎯 Bootstrap Execution Tracer");
    
    // Run the old bootstrap with full tracing
    traced_bootstrap!()?;
    
    println!("🎉 Bootstrap tracing completed!");
    Ok(())
}
