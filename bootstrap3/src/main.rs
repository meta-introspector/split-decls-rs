use std::path::Path;
use bootstrap3::bootstrap_from_output2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output2_dir = Path::new("../output2");
    let output3_dir = Path::new("../output3");
    
    println!("🔧 Bootstrap3: Pure output2 → output3 generation");
    println!("📂 Input:  {}", output2_dir.display());
    println!("📂 Output: {}", output3_dir.display());
    
    bootstrap_from_output2(output2_dir, output3_dir)?;
    
    println!("✨ Bootstrap3 completed successfully!");
    Ok(())
}
