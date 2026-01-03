use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use unified_build::transform_bootstrap::add_bootstrap_features;
use unified_build::transform_jobserver::fix_jobserver_imports;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustc_path = std::env::args().nth(1)
        .unwrap_or_else(|| "../submodules/rust".to_string());
    
    let output_path = std::env::args().nth(2)
        .unwrap_or_else(|| "./output".to_string());

    println!("🔄 Processing rustc source files from: {}", rustc_path);
    println!("📁 Output directory: {}", output_path);
    
    // Create output directory
    fs::create_dir_all(&output_path)?;
    
    let mut processed_count = 0;

    // Walk through all .rs files in rustc source
    for entry in WalkDir::new(&rustc_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let relative_path = entry.path().strip_prefix(&rustc_path)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();

            // Apply transformations
            let mut transformed = content;
            transformed = add_bootstrap_features(&transformed);
            transformed = fix_jobserver_imports(&transformed);

            // Write processed file
            let output_file = Path::new(&output_path).join("processed").join(&relative_path);
            
            if let Some(parent) = output_file.parent() {
                fs::create_dir_all(parent)?;
            }

            let processed_content = format!(
                "// PROCESSED BY: unified-build\n// SOURCE: {}\n\n{}",
                relative_path,
                transformed
            );

            fs::write(output_file, processed_content)?;
            processed_count += 1;
            
            if processed_count % 100 == 0 {
                println!("📊 Processed {} files", processed_count);
            }
        }
    }

    println!("✅ Processed {} files", processed_count);
    
    Ok(())
}
