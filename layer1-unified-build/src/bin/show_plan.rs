use unified_build::crate_processor::CrateProcessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_path = std::env::args().nth(1)
        .unwrap_or_else(|| "./output".to_string());

    println!("🎯 UNIFIED PROCESSOR - CRATE PLAN & PROGRESS");
    println!("📁 Processing directory: {}", output_path);
    println!("{}", "═".repeat(60));
    
    let mut processor = CrateProcessor::new();
    processor.discover_crates(&format!("{}/processed", output_path))?;
    processor.process_in_topological_order()?;
    
    Ok(())
}
