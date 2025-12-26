use anyhow::Result;
use std::path::Path;
use toml::Value;

/// Read and parse a Cargo.toml file
pub fn read_manifest(path: &Path) -> Result<Value> {
    let content = std::fs::read_to_string(path)?;
    let manifest: Value = toml::from_str(&content)?;
    Ok(manifest)
}

/// Write a Cargo.toml file
pub fn write_manifest(path: &Path, manifest: &Value) -> Result<()> {
    let content = toml::to_string_pretty(manifest)?;
    std::fs::write(path, content)?;
    Ok(())
}

/// Update manifest edition
pub fn update_edition(manifest: &mut Value, edition: &str) -> Result<()> {
    if let Some(package) = manifest.get_mut("package") {
        if let Some(package_table) = package.as_table_mut() {
            package_table.insert("edition".to_string(), Value::String(edition.to_string()));
        }
    }
    Ok(())
}
