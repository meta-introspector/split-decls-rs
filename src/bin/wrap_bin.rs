use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[command(name = "wrap-bin")]
#[command(about = "Generate wrapped binary main.rs and Cargo.toml in output2")]
struct Args {
    /// Name of the binary to wrap (e.g., "split-decls-rs")
    binary_name: String,
    
    /// Output directory (default: output2)
    #[arg(short, long, default_value = "output2")]
    output_dir: PathBuf,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    if args.verbose {
        println!("Generating wrapped binary for: {}", args.binary_name);
        println!("Output directory: {}", args.output_dir.display());
    }
    
    // Create output directory structure
    let src_dir = args.output_dir.join("src");
    fs::create_dir_all(&src_dir)?;
    
    // Generate main.rs that uses wrapped library
    let main_content = generate_wrapped_main(&args.binary_name)?;
    let main_path = src_dir.join("main.rs");
    fs::write(&main_path, main_content)?;
    
    // Generate Cargo.toml for the wrapped binary
    let cargo_content = generate_wrapped_cargo_toml(&args.binary_name)?;
    let cargo_path = args.output_dir.join("Cargo.toml");
    fs::write(&cargo_path, cargo_content)?;
    
    if args.verbose {
        println!("✅ Generated wrapped binary files:");
        println!("  - {}", main_path.display());
        println!("  - {}", cargo_path.display());
    }
    
    Ok(())
}

fn generate_wrapped_main(binary_name: &str) -> Result<String> {
    let wrapped_crate_name = format!("wrapped_{}", binary_name.replace("-", "_"));
    
    Ok(format!(r#"// Generated wrapped main.rs for {}
// This calls the main function from the wrapped library

use anyhow::Result;

fn main() -> Result<()> {{
    // Import and call the wrapped main function
    {}::main()
}}
"#, binary_name, wrapped_crate_name))
}

fn generate_wrapped_cargo_toml(binary_name: &str) -> Result<String> {
    let wrapped_crate_name = format!("wrapped_{}", binary_name.replace("-", "_"));
    
    Ok(format!(r#"[package]
name = "{}-wrapped"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "{}"
path = "src/main.rs"

[dependencies]
{} = {{ path = "./{}" }}
anyhow = "1.0"
"#, binary_name, binary_name, wrapped_crate_name, wrapped_crate_name))
}
