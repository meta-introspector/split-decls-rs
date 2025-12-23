use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Recursively copies contents from `src` to `dst`, with optional `ignore` patterns.
///
/// `ignore`: A list of file or directory names (relative to `src`) to ignore during the copy.
/// `overwrite`: If true, existing files in `dst` will be overwritten.
pub fn copy_dir_recursive(
    src: &Path,
    dst: &Path,
    ignore: &[String],
    overwrite: bool,
) -> Result<()> {
    fs::create_dir_all(dst).context(format!("Failed to create destination directory {}", dst.display()))?;

    for entry in fs::read_dir(src).context(format!("Failed to read source directory {}", src.display()))? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry_path.file_name().ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;
        let dest_path = dst.join(file_name);

        // Check if the current entry should be ignored
        if ignore.contains(&file_name.to_string_lossy().to_string()) {
            continue;
        }

        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path, ignore, overwrite)?;
        } else {
            if dest_path.exists() && !overwrite {
                continue; // Skip if file exists and overwrite is false
            }
            fs::copy(&entry_path, &dest_path)
                .context(format!("Failed to copy file from {} to {}", entry_path.display(), dest_path.display()))?;
        }
    }
    Ok(())
}

