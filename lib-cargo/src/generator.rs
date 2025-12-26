use anyhow::Result;
use std::path::Path;
use toml::Value;

/// Generate a new Cargo.toml for a wrapped crate
pub fn generate_wrapped_cargo_toml(
    original_cargo_path: &Path,
    output_cargo_path: &Path,
    crate_name: &str,
) -> Result<()> {
    let mut manifest = crate::manifest::read_manifest(original_cargo_path)?;
    
    // Update package name
    if let Some(package) = manifest.get_mut("package") {
        if let Some(package_table) = package.as_table_mut() {
            package_table.insert("name".to_string(), Value::String(format!("wrapped-{}", crate_name)));
        }
    }
    
    // Update edition to 2024
    crate::manifest::update_edition(&mut manifest, "2024")?;
    
    // Convert dependencies to workspace format
    convert_deps_to_workspace(&mut manifest)?;
    
    crate::manifest::write_manifest(output_cargo_path, &manifest)?;
    Ok(())
}

/// Convert all dependencies to use workspace = true
fn convert_deps_to_workspace(manifest: &mut Value) -> Result<()> {
    let special_crates = ["alloc", "core", "std", "proc_macro"];
    
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(deps) = manifest.get_mut(section).and_then(|v| v.as_table_mut()) {
            for (name, value) in deps.iter_mut() {
                if special_crates.contains(&name.as_str()) {
                    // Keep special crates as-is, don't convert to workspace
                    continue;
                }
                
                if let Some(table) = value.as_table_mut() {
                    // Remove version/path and add workspace = true
                    table.remove("version");
                    table.remove("path");
                    table.insert("workspace".to_string(), Value::Boolean(true));
                }
            }
        }
    }
    Ok(())
}
