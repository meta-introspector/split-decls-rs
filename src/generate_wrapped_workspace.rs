use crate::syscall;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use toml::Table;

use crate::patch_config;
use split_decls_types::SplitDeclsConfig;
use crate::format_generated_rust_files::format_generated_rust_files;
use crate::wrapped_workspace_handlers::{single_crate, multi_crate};
use crate::path_diff;
use crate::add_generated_header;

#[doc = " Generates a new workspace containing \"wrapped\" versions of the target crates."]
#[doc = " Each wrapped crate will have its declarations eagerly split and patched."]
pub fn generate_wrapped_workspace(
    output_dir: &Path,
    patch_config: &patch_config::PatchConfig,
    global_config: &SplitDeclsConfig,
    scan_root: &Path,
    dry_run: bool,
    verbose: bool,
    cargo_only: bool,
) -> Result<Vec<crate::eager_splitter::ModuleNotFoundReport>> { // Changed return type
    if verbose {
        println!(
            "DEBUG: generate_wrapped_workspace called with output_dir: {}",
            output_dir.display()
        );
        println!(
            "DEBUG: Scanning root for Cargo.tomls: {}",
            scan_root.display()
        );
        println!(
            "DEBUG: patch_config.generated_workspace_member.is_empty(): {}",
            patch_config.generated_workspace_member.is_empty()
        );
    }
    if !dry_run {
        fs::create_dir_all(output_dir).context(format!(
            "Failed to create wrapped workspace directory: {}",
            output_dir.display()
        ))?;
    }
    if verbose {
        println!("Wrapped workspace directory: {}", output_dir.display());
    }

    let workspace_cargo_toml_path = output_dir.join("Cargo.toml");
    let final_cargo_toml_content;
    let collected_errors: Vec<crate::eager_splitter::ModuleNotFoundReport>; // Declare to hold errors

    let main_crate_cargo_toml_path = scan_root.join("Cargo.toml");

    // When generating a wrapped workspace, the output_dir's Cargo.toml should always be a workspace root.
    // Individual packages within the workspace will have their own Cargo.toml files.
    let (content, errors) = multi_crate::handle_multi_crate_wrapping(
        output_dir,
        patch_config,
        global_config,
        scan_root,
        dry_run,
        verbose,
        cargo_only,
    )?; // Capture both content and errors
    final_cargo_toml_content = content;
    collected_errors = errors;
    
    if !dry_run {
        add_generated_header!(
            &workspace_cargo_toml_path,
            final_cargo_toml_content.as_str(),
            file!(),
            line!()
        )
        .context(format!(
            "Failed to write Cargo.toml for workspace: {}",
            workspace_cargo_toml_path.display()
        ))?;
        format_generated_rust_files(output_dir, verbose)?;
    }

    if verbose {
        println!(
            "Generated Cargo.toml at: {}",
            workspace_cargo_toml_path.display()
        );
        println!("DEBUG: Exiting generate_wrapped_workspace.");
    }
    Ok(collected_errors) // Return collected errors
}
