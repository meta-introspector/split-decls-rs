// Generated from: ./src/generate_new_workspace.rs
// Original file: ./src/generate_new_workspace.rs
// Function: generate_root_toml

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

#[decl_split_decls_rs_generate_new_workspace]
# [doc = " Generates a Cargo.toml for a new workspace root."] # [doc = " Takes the output directory and a list of paths to the member crates."] pub fn generate_root_toml (output_dir : & Path , crate_paths : & [PathBuf] , dry_run : bool ,) -> Result < () > { let mut workspace_members_content = Vec :: new () ; for crate_path in crate_paths { let relative_path = crate_path . strip_prefix (output_dir) . unwrap_or (crate_path) . display () . to_string () ; workspace_members_content . push (format ! ("\"{}\"" , relative_path)) ; } let workspace_cargo_toml_content = format ! (r#"[workspace]\nresolver = \"2\"\nmembers = [\n    {}\n]\n"# , workspace_members_content . join (",\n    ") ,) ; let workspace_cargo_toml_path = output_dir . join ("Cargo.toml") ; if dry_run { println ! ("Dry-run: Would write workspace Cargo.toml to {}" , workspace_cargo_toml_path . display ()) ; println ! ("{}" , workspace_cargo_toml_content) ; } else { add_generated_header ! (& workspace_cargo_toml_path , workspace_cargo_toml_content . as_str () , file ! () , line ! ()) . context (format ! ("Failed to write Cargo.toml for new workspace: {}" , workspace_cargo_toml_path . display ())) ? ; println ! ("Generated workspace Cargo.toml at: {}" , workspace_cargo_toml_path . display ()) ; } Ok (()) }