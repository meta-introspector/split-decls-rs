use anyhow::Result;
use lib_cargo::*;
use std::path::PathBuf;
use walkdir::WalkDir;
use clap::Parser;

#[derive(clap::Parser)]
struct Cli {
    #[arg(long, default_value = "output2")]
    output_dir: PathBuf,
    #[arg(long)]
    verbose: bool,
    #[arg(long)]
    build: bool,
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("Regenerating Cargo.toml files in {}", cli.output_dir.display());
    }
    
    // Collect workspace members
    let members = collect_workspace_members(&cli.output_dir)?;
    
    // Regenerate individual Cargo.toml files
    let mut updated_crates = Vec::new();
    for entry in WalkDir::new(&cli.output_dir).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() && path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("wrapped-"))
            .unwrap_or(false) 
        {
            let crate_name = path.file_name().unwrap().to_str().unwrap()
                .strip_prefix("wrapped-").unwrap();
            
            if cli.verbose {
                println!("Updating Cargo.toml for {}", crate_name);
            }

            let submodule_path = PathBuf::from("submodules").join(crate_name);
            if submodule_path.exists() {
                let original_cargo = submodule_path.join("Cargo.toml");
                let output_cargo = path.join("Cargo.toml");
                
                if !cli.dry_run {
                    generate_wrapped_cargo_toml(&original_cargo, &output_cargo, crate_name)?;
                    updated_crates.push(path.to_path_buf());
                }
            }
        }
    }
    
    // Collect all dependencies from updated crates
    let all_deps = collect_all_workspace_dependencies(&cli.output_dir)?;
    
    // Generate workspace Cargo.toml
    generate_workspace_toml(&members, &all_deps, &cli.output_dir.join("Cargo.toml"))?;
    
    if cli.verbose {
        println!("✅ Cargo.toml regeneration complete");
    }
    
    // Build test if requested
    if cli.build && !cli.dry_run {
        test_build_crates(&updated_crates, cli.verbose)?;
    }
    
    Ok(())
}

fn test_build_crates(crates: &[PathBuf], verbose: bool) -> Result<()> {
    if verbose {
        println!("Building updated crates...");
    }
    
    let mut error_count = 0;
    for crate_path in crates {
        let output = std::process::Command::new("cargo")
            .args(&["check", "--quiet"])
            .current_dir(crate_path)
            .output();
            
        match output {
            Ok(result) if !result.status.success() => {
                error_count += 1;
                let crate_name = crate_path.file_name().unwrap().to_str().unwrap();
                println!("❌ {}: {}", crate_name, String::from_utf8_lossy(&result.stderr).lines().next().unwrap_or("Build failed"));
            }
            Err(e) => {
                error_count += 1;
                let crate_name = crate_path.file_name().unwrap().to_str().unwrap();
                println!("❌ {}: {}", crate_name, e);
            }
            _ => {} // Success
        }
    }
    
    if error_count > 0 {
        println!("❌ {} crates failed to build", error_count);
        std::process::exit(101);
    } else if verbose {
        println!("✅ All crates built successfully");
    }
    
    Ok(())
}
