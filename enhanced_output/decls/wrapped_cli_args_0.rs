// Generated from: ./src/cli_args.rs
// Original file: ./src/cli_args.rs
// Function: parse_args

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

#[decl_split_decls_rs_cli_args]
pub fn parse_args () -> Result < CliArgs > { let mut args : Vec < String > = std :: env :: args () . collect () ; let dry_run_index = args . iter () . position (| arg | arg == "--dry-run") ; let dry_run = dry_run_index . is_some () ; if let Some (index) = dry_run_index { args . remove (index) ; } let verbose_index = args . iter () . position (| arg | arg == "--verbose") ; let verbose = verbose_index . is_some () ; if let Some (index) = verbose_index { args . remove (index) ; } let mut wrapped_workspace_output_dir = PathBuf :: from ("output") ; let mut generate_wrapped_workspace_mode = true ; if let Some (index) = args . iter () . position (| arg | arg == "--no-wrapped-workspace") { generate_wrapped_workspace_mode = false ; args . remove (index) ; } if let Some (index) = args . iter () . position (| arg | arg == "--output-dir") { args . remove (index) ; if let Some (output_path_str) = args . get (index) { wrapped_workspace_output_dir = PathBuf :: from (output_path_str) ; args . remove (index) ; } else { anyhow :: bail ! ("--output-dir requires a path.") ; } } if let Some (index) = args . iter () . position (| arg | arg == "--generate-wrapped-workspace") { generate_wrapped_workspace_mode = true ; args . remove (index) ; if let Some (output_path_str) = args . get (index) { wrapped_workspace_output_dir = PathBuf :: from (output_path_str) ; args . remove (index) ; } else { } } let patch_config_path_str = args . get (1) . map_or ("patch.toml" . to_string () , | s | s . to_string ()) ; Ok (CliArgs { dry_run , verbose , wrapped_workspace_output_dir , patch_config_path_str , generate_wrapped_workspace_mode , }) }