// Generated from: ./src/backup_original_cargo.rs
// Original file: ./src/backup_original_cargo.rs
// Function: backup_original_cargotoml

use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use split_decls_types::SplitDeclsConfig;
pub use extracted_decl::*;
pub use process_crate::process_crate;
pub use process_crates_in_path::process_crates_in_path;
pub use generate_wrapped_workspace::generate_wrapped_workspace;
prelude!{}

#[decl_split_decls_rs_backup_original_cargo]
# [doc = " Backs up original Cargo.toml file."] pub fn backup_original_cargotoml (paths : & CratePaths , dry_run : bool) -> Result < () > { if dry_run { println ! ("Dry-run: Would have backed up {} to {}" , paths . cargo_toml_path . display () , paths . old_cargo_toml_path . display ()) ; return Ok (()) ; } if paths . cargo_toml_path . exists () && ! paths . old_cargo_toml_path . exists () { fs :: rename (& paths . cargo_toml_path , & paths . old_cargo_toml_path) . context (format ! ("Failed to rename {} to {}" , paths . cargo_toml_path . display () , paths . old_cargo_toml_path . display ())) ? ; println ! ("Renamed {} to {}" , paths . cargo_toml_path . display () , paths . old_cargo_toml_path . display ()) ; } else if paths . old_cargo_toml_path . exists () { println ! ("Backup {} already exists, skipping rename of {}" , paths . old_cargo_toml_path . display () , paths . cargo_toml_path . display ()) ; } else if ! paths . cargo_toml_path . exists () { fs :: write (& paths . old_cargo_toml_path , "") ? ; println ! ("No {} found, created empty {}" , paths . cargo_toml_path . display () , paths . old_cargo_toml_path . display ()) ; } Ok (()) }