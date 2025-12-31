use split_decls_genesis::*;

fn main() -> anyhow::Result<()> {
    println!("🌟 Split-Decls Genesis System");
    
    run_system()?;
    
    println!("✅ System completed successfully!");
    Ok(())
}
