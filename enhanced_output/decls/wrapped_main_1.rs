// Generated from: ./src/main.rs
// Original file: ./src/main.rs
// Function: run_ecosystem_scan_mode

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

#[decl_split_decls_rs_main]
fn run_ecosystem_scan_mode (verbose : bool , dry_run : bool , base_path : & Path , recursive : bool , global_config : & SplitDeclsConfig ,) -> Result < () > { ecosystem_processor :: process_ecosystem (verbose , dry_run , base_path , recursive , global_config) }