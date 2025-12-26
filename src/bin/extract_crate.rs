use anyhow::Result;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

/// Extract a single crate with its dependencies into a standalone workspace
#[derive(Parser, Debug)]
#[clap(author, version, about = "Extract crate with dependencies for standalone checkout")]
struct Args {
    /// Name of the crate to extract (e.g., "addr2line", "serde")
    crate_name: String,

    /// Output directory for standalone workspace
    #[clap(short, long, default_value = "extracted")]
    output: PathBuf,

    /// Include dev-dependencies
    #[clap(long)]
    include_dev_deps: bool,

    /// Maximum dependency depth (0 = direct deps only)
    #[clap(long, default_value = "3")]
    max_depth: usize,

    /// Verbose output
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone)]
struct CrateInfo {
    name: String,
    path: PathBuf,
    dependencies: HashSet<String>,
    dev_dependencies: HashSet<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("🔍 Extracting {} with dependencies to {}", 
        args.crate_name, args.output.display());
    
    // Find the target crate in output2
    let output2_path = Path::new("output2");
    let target_crate = find_wrapped_crate(&output2_path, &args.crate_name)?;
    
    if args.verbose {
        println!("📦 Found target crate: {}", target_crate.path.display());
    }
    
    // Discover all dependencies recursively
    let mut dependency_graph = HashMap::new();
    let mut all_crates = HashSet::new();
    
    discover_dependencies(
        &output2_path,
        &target_crate,
        &mut dependency_graph,
        &mut all_crates,
        args.max_depth,
        args.include_dev_deps,
        args.verbose,
    )?;
    
    println!("📊 Discovered {} total crates (including dependencies)", all_crates.len());
    
    // Create standalone workspace
    create_standalone_workspace(
        &args.output,
        &target_crate,
        &all_crates,
        &dependency_graph,
        args.verbose,
    )?;
    
    println!("✅ Standalone workspace created at {}", args.output.display());
    println!("   Main crate: {}", args.crate_name);
    println!("   Total crates: {}", all_crates.len());
    println!("   Ready for: cd {} && cargo build", args.output.display());
    
    Ok(())
}

/// Find wrapped crate by name in output2
fn find_wrapped_crate(output2_path: &Path, crate_name: &str) -> Result<CrateInfo> {
    let wrapped_name = format!("wrapped-{}", crate_name);
    let crate_path = output2_path.join(&wrapped_name);
    
    if !crate_path.exists() {
        return Err(anyhow::anyhow!("Wrapped crate not found: {}", wrapped_name));
    }
    
    let cargo_toml = crate_path.join("Cargo.toml");
    let (deps, dev_deps) = parse_dependencies(&cargo_toml)?;
    
    Ok(CrateInfo {
        name: wrapped_name,
        path: crate_path,
        dependencies: deps,
        dev_dependencies: dev_deps,
    })
}

/// Parse dependencies from Cargo.toml
fn parse_dependencies(cargo_toml: &Path) -> Result<(HashSet<String>, HashSet<String>)> {
    let content = fs::read_to_string(cargo_toml)?;
    let toml: Value = toml::from_str(&content)?;
    
    let mut deps = HashSet::new();
    let mut dev_deps = HashSet::new();
    
    // Parse [dependencies]
    if let Some(dependencies) = toml.get("dependencies").and_then(|v| v.as_table()) {
        for (name, _) in dependencies {
            deps.insert(name.clone());
        }
    }
    
    // Parse [dev-dependencies]
    if let Some(dev_dependencies) = toml.get("dev-dependencies").and_then(|v| v.as_table()) {
        for (name, _) in dev_dependencies {
            dev_deps.insert(name.clone());
        }
    }
    
    Ok((deps, dev_deps))
}

/// Recursively discover all dependencies
fn discover_dependencies(
    output2_path: &Path,
    crate_info: &CrateInfo,
    dependency_graph: &mut HashMap<String, CrateInfo>,
    all_crates: &mut HashSet<String>,
    max_depth: usize,
    include_dev_deps: bool,
    verbose: bool,
) -> Result<()> {
    if max_depth == 0 || all_crates.contains(&crate_info.name) {
        return Ok(());
    }
    
    all_crates.insert(crate_info.name.clone());
    dependency_graph.insert(crate_info.name.clone(), crate_info.clone());
    
    if verbose {
        println!("  📋 Processing: {} ({} deps)", 
            crate_info.name, crate_info.dependencies.len());
    }
    
    // Process direct dependencies
    for dep_name in &crate_info.dependencies {
        if let Ok(dep_crate) = find_wrapped_crate(output2_path, dep_name) {
            discover_dependencies(
                output2_path,
                &dep_crate,
                dependency_graph,
                all_crates,
                max_depth - 1,
                include_dev_deps,
                verbose,
            )?;
        }
    }
    
    // Process dev-dependencies if requested
    if include_dev_deps {
        for dep_name in &crate_info.dev_dependencies {
            if let Ok(dep_crate) = find_wrapped_crate(output2_path, dep_name) {
                discover_dependencies(
                    output2_path,
                    &dep_crate,
                    dependency_graph,
                    all_crates,
                    max_depth - 1,
                    include_dev_deps,
                    verbose,
                )?;
            }
        }
    }
    
    Ok(())
}

/// Create standalone workspace with extracted crates
fn create_standalone_workspace(
    output_path: &Path,
    target_crate: &CrateInfo,
    all_crates: &HashSet<String>,
    dependency_graph: &HashMap<String, CrateInfo>,
    verbose: bool,
) -> Result<()> {
    // Create output directory
    fs::create_dir_all(output_path)?;
    
    // Copy all crates
    for crate_name in all_crates {
        if let Some(crate_info) = dependency_graph.get(crate_name) {
            let dest_path = output_path.join(&crate_info.name);
            
            if verbose {
                println!("  📁 Copying: {} → {}", 
                    crate_info.path.display(), dest_path.display());
            }
            
            copy_dir_recursive(&crate_info.path, &dest_path)?;
        }
    }
    
    // Generate workspace Cargo.toml
    let workspace_toml = generate_workspace_toml(target_crate, all_crates, dependency_graph)?;
    fs::write(output_path.join("Cargo.toml"), workspace_toml)?;
    
    // Create README for the extracted workspace
    let readme = generate_extracted_readme(target_crate, all_crates)?;
    fs::write(output_path.join("README.md"), readme)?;
    
    Ok(())
}

/// Generate workspace Cargo.toml for extracted crates
fn generate_workspace_toml(
    target_crate: &CrateInfo,
    all_crates: &HashSet<String>,
    dependency_graph: &HashMap<String, CrateInfo>,
) -> Result<String> {
    let mut toml = String::new();
    
    toml.push_str(&format!(r#"# Extracted Workspace for {}
# Generated by split-decls-rs extract tool
# Main crate: {}

[workspace]
resolver = "2"
members = [
"#, target_crate.name, target_crate.name));
    
    // Add all crates as workspace members
    for crate_name in all_crates {
        toml.push_str(&format!("    \"{}\",\n", crate_name));
    }
    
    toml.push_str("]\n\n");
    
    // Add workspace package defaults
    toml.push_str(r#"[workspace.package]
edition = "2024"
version = "0.1.0"
authors = ["Split-Decls-RS Extract Tool"]
license = "MIT OR Apache-2.0"
description = "Extracted standalone workspace"
repository = "https://github.com/your-org/extracted-workspace"

[workspace.lints]
# Workspace-level lints

"#);
    
    // Add workspace dependencies
    toml.push_str("[workspace.dependencies]\n");
    
    // Collect all unique dependencies
    let mut all_deps = HashSet::new();
    for crate_info in dependency_graph.values() {
        all_deps.extend(&crate_info.dependencies);
        all_deps.extend(&crate_info.dev_dependencies);
    }
    
    for dep in &all_deps {
        if all_crates.contains(&format!("wrapped-{}", dep)) {
            // Internal workspace dependency
            toml.push_str(&format!("{} = {{ path = \"wrapped-{}\" }}\n", dep, dep));
        } else {
            // External dependency
            toml.push_str(&format!("{} = \"*\"\n", dep));
        }
    }
    
    Ok(toml)
}

/// Generate README for extracted workspace
fn generate_extracted_readme(
    target_crate: &CrateInfo,
    all_crates: &HashSet<String>,
) -> Result<String> {
    let readme = format!(r#"# Extracted Workspace: {}

This is a standalone workspace extracted from split-decls-rs containing the `{}` crate and all its dependencies.

## Contents

- **Main Crate**: `{}`
- **Total Crates**: {}
- **Dependencies**: All required dependencies included

## Usage

```bash
# Build the main crate
cargo build -p {}

# Build entire workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Check all crates
cargo check --workspace
```

## Crates Included

{}

## Generated by

This workspace was generated by the split-decls-rs extract tool, which creates standalone, self-contained workspaces from wrapped Rust crates.

For more information, see: https://github.com/your-org/split-decls-rs
"#,
        target_crate.name,
        target_crate.name,
        target_crate.name,
        all_crates.len(),
        target_crate.name,
        all_crates.iter()
            .map(|name| format!("- `{}`", name))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    Ok(readme)
}

/// Recursively copy directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dependency_parsing() {
        // Test parsing Cargo.toml dependencies
        let toml_content = r#"
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
criterion = "0.4"
"#;
        
        let toml: Value = toml::from_str(toml_content).unwrap();
        // Test implementation would go here
    }
    
    #[test]
    fn test_workspace_generation() {
        // Test workspace Cargo.toml generation
        let mut all_crates = HashSet::new();
        all_crates.insert("wrapped-addr2line".to_string());
        all_crates.insert("wrapped-serde".to_string());
        
        // Test implementation would go here
    }
}
