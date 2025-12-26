// Generated from: ./src/buildrs_generator/main_logic.rs
// Original file: ./src/buildrs_generator/main_logic.rs
// Function: generate_main_logic_token_stream

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

#[decl_split_decls_rs_main_logic]
pub fn generate_main_logic_token_stream (_decls_output_dir_lit : & LitStr , crate_name_sanitized_lit : & LitStr ,) -> TokenStream { quote ! { fn main () -> Result < () > { println ! ("cargo:rerun-if-changed=build.rs") ; println ! ("cargo:rerun-if-changed=.split-decls-config.toml") ; let config_path = PathBuf :: from (std :: env :: var ("CARGO_MANIFEST_DIR") ?) . join (".split-decls-config.toml") ; let config = SplitDeclsConfig :: load_from_file (& config_path) . context (format ! ("Failed to load config from {}" , config_path . display ())) ?; let current_crate_name_for_patch = # crate_name_sanitized_lit . to_string () ; if let Some (patches_for_crate) = config . patches . get (& current_crate_name_for_patch) { for patch_spec in patches_for_crate { let patch_path = PathBuf :: from (std :: env :: var ("CARGO_MANIFEST_DIR") ?) . join (& patch_spec . path) ; println ! ("cargo:rerun-if-changed={}" , patch_path . display ()) ; } } println ! ("cargo:note=build.rs finished. If `split-decls-rs` needs to be re-run, changes will be detected.") ; Ok (()) } } }