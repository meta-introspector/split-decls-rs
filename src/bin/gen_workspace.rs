use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use split_decls_rs::config_macros::GLOBAL_CONFIG;

#[derive(Parser)]
#[command(name = "gen-workspace")]
#[command(about = "Generate workspace Cargo.toml using split-decls-rs.toml configuration")]
struct Cli {
    /// Output directory (default: output2)
    #[arg(short, long, default_value = "output2")]
    output_dir: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Use the existing configuration system
    let config = GLOBAL_CONFIG.lock().unwrap();
    
    // Collect all wrapped crate names (members)
    let mut members = Vec::new();
    for entry in WalkDir::new(&cli.output_dir).max_depth(1) {
        let entry = entry?;
        if entry.path().is_dir() {
            if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                if name.starts_with("wrapped-") {
                    members.push(format!("\"{}\"", name));
                }
            }
        }
    }
    members.sort();
    
    // Generate workspace dependencies from configuration
    let mut workspace_deps = HashMap::new();
    
    for crate_name in &config.wrapping.crates {
        if let Some(path_override) = config.crate_path_overrides.as_ref().and_then(|map| map.get(crate_name)) {
            if path_override.to_string_lossy() == "*" {
                // External crate - use version override or default
                if let Some(overrides) = &config.workspace_dependency_overrides {
                    if let Some(override_spec) = overrides.get(crate_name) {
                        let toml_string = toml::to_string(override_spec)?;
                        workspace_deps.insert(crate_name.clone(), toml_string.trim().to_string());
                    } else {
                        workspace_deps.insert(crate_name.clone(), "{ version = \"*\" }".to_string());
                    }
                } else {
                    workspace_deps.insert(crate_name.clone(), "{ version = \"*\" }".to_string());
                }
            } else {
                // Local crate - convert absolute path to relative
                let relative_path = if path_override.to_string_lossy().contains("/submodules/") {
                    let parts: Vec<&str> = path_override.to_string_lossy().split("/submodules/").collect();
                    if parts.len() == 2 {
                        format!("../submodules/{}", parts[1])
                    } else {
                        format!("../{}", crate_name) // fallback
                    }
                } else {
                    format!("../{}", crate_name) // fallback for non-submodule paths
                };
                workspace_deps.insert(crate_name.clone(), format!("{{ path = \"{}\" }}", relative_path));
            }
        } else {
            // No override, assume local submodule
            workspace_deps.insert(crate_name.clone(), format!("{{ path = \"../submodules/{}\" }}", crate_name));
        }
    }
    
    // Apply workspace dependency overrides
    if let Some(overrides) = &config.workspace_dependency_overrides {
        for (name, override_spec) in overrides {
            let toml_string = toml::to_string(override_spec)?;
            workspace_deps.insert(name.clone(), toml_string.trim().to_string());
        }
    }
    
    // Generate workspace Cargo.toml
    let mut workspace_content = String::new();
    workspace_content.push_str("# Generated workspace Cargo.toml from split-decls-rs.toml configuration\n\n");
    workspace_content.push_str("[workspace]\n");
    workspace_content.push_str("resolver = \"2\"\n");
    workspace_content.push_str("members = [\n");
    for member in &members {
        workspace_content.push_str(&format!("    {},\n", member));
    }
    workspace_content.push_str("]\n\n");
    
    workspace_content.push_str("[workspace.dependencies]\n");
    let mut sorted_deps: Vec<_> = workspace_deps.iter().collect();
    sorted_deps.sort_by_key(|(name, _)| *name);
    
    for (name, spec) in sorted_deps {
        workspace_content.push_str(&format!("{} = {}\n", name, spec));
    }
    
    // Write workspace Cargo.toml
    let workspace_path = cli.output_dir.join("Cargo.toml");
    fs::write(&workspace_path, workspace_content)?;
    
    println!("✅ Generated workspace with {} members and {} dependencies using split-decls-rs.toml", members.len(), workspace_deps.len());
    
    Ok(())
}
