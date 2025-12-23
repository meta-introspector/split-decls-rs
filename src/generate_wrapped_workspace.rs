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
) -> Result<()> {
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

    let main_crate_cargo_toml_path = scan_root.join("Cargo.toml");

    // Condition to determine single vs multi-crate handling
    if patch_config.generated_workspace_member.is_empty() && main_crate_cargo_toml_path.exists() {
        if verbose {
            println!(
                "DEBUG: Detected single crate wrapping scenario. Generating [package] Cargo.toml."
            );
        }
        let root_cargo_toml_content =
            fs::read_to_string(&main_crate_cargo_toml_path).context(format!(
                "Failed to read Cargo.toml from scan_root: {}",
                main_crate_cargo_toml_path.display()
            ))?;
        let root_cargo_toml: Table = toml::from_str(&root_cargo_toml_content).context(format!(
            "Failed to parse Cargo.toml from scan_root: {}",
            main_crate_cargo_toml_path.display()
        ))?;
        
        final_cargo_toml_content = single_crate::handle_single_crate_wrapping(
            output_dir,
            patch_config,
            global_config,
            scan_root,
            dry_run,
            verbose,
            &main_crate_cargo_toml_path,
            &root_cargo_toml,
        )?;
    } else {
        final_cargo_toml_content = multi_crate::handle_multi_crate_wrapping(
            output_dir,
            patch_config,
            global_config,
            scan_root,
            dry_run,
            verbose,
        )?;
    }
    
    if !dry_run {
        add_generated_header!(
            &workspace_cargo_toml_path,
            final_cargo_toml_content.as_str()
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
    Ok(())
}
