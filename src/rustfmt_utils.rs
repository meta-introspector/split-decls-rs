use anyhow::{Context, Result};
use std::path::Path;
use std::fs;

pub fn format_rust_file(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let syntax_tree: syn::File = syn::parse_str(&content)
        .context(format!("Failed to parse file: {}", path.display()))?;
    let formatted = prettyplease::unparse(&syntax_tree);
    fs::write(path, formatted)?;
    Ok(())
}
