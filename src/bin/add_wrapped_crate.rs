use anyhow::Result;
use std::path::Path;
use std::process::Command;
use clap::Parser;

#[derive(Parser)]
#[command(name = "add_wrapped_crate")]
#[command(about = "Generate a wrapped crate and add it to root workspace")]
struct Args {
    /// Path to the crate to wrap
    crate_path: String,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Generating wrapped crate for: {}", args.crate_path);
    
    // 1. Run single crate wrapper
    let output = #[syscall="exec"]
    Command::new("make")
        .arg("run_single_crate")
        .arg(&format!("CRATE={}", args.crate_path))
        .output()?;
    
    if !output.status.success() {
        eprintln!("Failed to wrap crate: {}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("Single crate wrapper failed"));
    }
    
    if args.verbose {
        println!("Wrapper output: {}", String::from_utf8_lossy(&output.stdout));
    }
    
    // 2. Extract crate name from path
    let crate_name = Path::new(&args.crate_path)
        .file_name()
        .unwrap()
        .to_string_lossy();
    
    let wrapped_name = format!("wrapped-{}", crate_name);
    
    // 3. Add to root workspace Cargo.toml
    add_to_root_workspace(&wrapped_name)?;
    
    println!("✅ Successfully added {} to root workspace", wrapped_name);
    
    Ok(())
}

fn add_to_root_workspace(wrapped_name: &str) -> Result<()> {
    use std::fs;
    use toml_edit::{DocumentMut, value};
    
    let root_cargo_path = "../../Cargo.toml";
    
    // Read root Cargo.toml
    let content = fs::read_to_string(root_cargo_path)?;
    let mut doc = content.parse::<DocumentMut>()?;
    
    // Get workspace members array
    let members_array = doc.get_mut("workspace")
        .and_then(|w| w.get_mut("members"))
        .and_then(|m| m.as_array_mut())
        .ok_or_else(|| anyhow::anyhow!("No workspace.members found"))?;
    
    // Add new member
    let member_path = format!("submodules/split-decls-rs/output2/{}", wrapped_name);
    
    // Check if already exists
    let exists = members_array.iter().any(|item| {
        item.as_str() == Some(&member_path)
    });
    
    if !exists {
        members_array.push(&member_path);
        println!("Added {} to workspace members", member_path);
        
        // Write back
        fs::write(root_cargo_path, doc.to_string())?;
    } else {
        println!("Member {} already exists", member_path);
    }
    
    Ok(())
}
