// Generated from: ./src/bin/regen_cargo_v2.rs
// Original file: ./src/bin/regen_cargo_v2.rs
// Function: main

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

#[decl_split_decls_rs_regen_cargo_v2]
fn main () -> Result < () > { let cli = Cli :: parse () ; let content = std :: fs :: read_to_string ("split-decls-rs.toml") . unwrap_or_default () ; let config : split_decls_types :: SplitDeclsConfig = if content . is_empty () { split_decls_types :: SplitDeclsConfig :: default () } else { toml :: from_str (& content) . unwrap () } ; if cli . verbose { println ! ("Regenerating Cargo.toml files in {}" , cli . output_dir . display ()) ; } let members = collect_workspace_members (& cli . output_dir) ? ; let mut updated_crates = Vec :: new () ; for entry in WalkDir :: new (& cli . output_dir) . max_depth (1) { let entry = entry ? ; let path = entry . path () ; if path . is_dir () && path . file_name () . and_then (| n | n . to_str ()) . map (| s | s . starts_with ("wrapped-")) . unwrap_or (false) { let crate_name = path . file_name () . unwrap () . to_str () . unwrap () . strip_prefix ("wrapped-") . unwrap () ; if cli . verbose { println ! ("Updating Cargo.toml for {}" , crate_name) ; } let submodule_path = PathBuf :: from ("submodules") . join (crate_name) ; if submodule_path . exists () { let original_cargo = submodule_path . join ("Cargo.toml") ; let output_cargo = path . join ("Cargo.toml") ; if ! cli . dry_run { generate_wrapped_cargo_toml (& original_cargo , & output_cargo , crate_name) ? ; updated_crates . push (path . to_path_buf ()) ; } } } } let mut all_deps = collect_all_workspace_dependencies (& cli . output_dir) ? ; println ! ("Config loaded, crate_path_overrides: {:?}" , config . crate_path_overrides . is_some ()) ; if let Some (overrides) = & config . crate_path_overrides { println ! ("Found {} package aliases" , overrides . len ()) ; for (name , path) in overrides { println ! ("Package alias: {} = {}" , name , path . display ()) ; let mut workspace_entry = lib_cargo :: toml :: map :: Map :: new () ; workspace_entry . insert ("path" . to_string () , lib_cargo :: toml :: Value :: String (path . to_string_lossy () . to_string ())) ; all_deps . insert (name . clone () , lib_cargo :: toml :: Value :: Table (workspace_entry)) ; } } else { println ! ("No crate_path_overrides found in config") ; } generate_workspace_toml (& members , & all_deps , & cli . output_dir . join ("Cargo.toml")) ? ; if cli . verbose { println ! ("✅ Cargo.toml regeneration complete") ; } if cli . build && ! cli . dry_run { test_build_crates (& updated_crates , cli . verbose) ? ; } Ok (()) }