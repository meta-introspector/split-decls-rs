use anyhow::{Context, Result};
use std::path::Path;
use std::panic::{catch_unwind, UnwindSafe};

pub fn format_rust_file(content: &str, path: &Path) -> Result<String> {
    // Formatting disabled to avoid edition compatibility issues
    if std::env::var("SPLIT_DECLS_DEBUG").is_ok() {
        eprintln!("DEBUG: Skipping formatting for {}", path.display());
    }
    Ok(content.to_string())
    
    /* Original formatting code disabled:
    let syntax_tree: syn::File = syn::parse_str(content)
        .context(format!("Failed to parse file: {}", path.display()))?;
    
    let formatted = catch_unwind(move || {
        prettyplease::unparse(&syntax_tree)
    }).map_err(|e| {
        let panic_msg = if let Some(s) = e.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = e.downcast_ref::<&str>() {
            s.to_string()
        } else {
            "An unknown panic occurred during prettyplease::unparse".to_string()
        };
        anyhow::anyhow!("prettyplease::unparse panicked for file {}: {}", path.display(), panic_msg)
    })?;
    Ok(formatted)
    */
}
