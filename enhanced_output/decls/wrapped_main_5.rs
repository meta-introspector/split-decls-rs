// Generated from: ./src/main.rs
// Original file: ./src/main.rs
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

#[decl_split_decls_rs_main]
fn main () -> Result < () > { Builder :: from_default_env () . write_style (WriteStyle :: Always) . filter_level (log :: LevelFilter :: Info) . init () ; load_config ! ("split-decls-rs.toml") ; let cli = Cli :: parse () ; if cli . verbose { info ! ("Verbose mode enabled.") ; info ! ("CLI args: {:?}" , cli) ; } match & cli . command { Commands :: WrappedWorkspace { output_dir , dry_run } => { run_wrapped_workspace_mode (cli . verbose , * dry_run , output_dir . as_ref () , & split_decls_rs :: config_macros :: GLOBAL_CONFIG . lock () . unwrap () , false) ? ; } Commands :: EcosystemScan { base_path , recursive , dry_run } => { run_ecosystem_scan_mode (cli . verbose , * dry_run , base_path , * recursive , & split_decls_rs :: config_macros :: GLOBAL_CONFIG . lock () . unwrap ()) ? ; } Commands :: ExecuteGoalWorkflow { goal_file , dry_run } => { run_execute_goal_workflow_mode (cli . verbose , * dry_run , goal_file , & split_decls_rs :: config_macros :: GLOBAL_CONFIG . lock () . unwrap ()) ? ; } Commands :: Bootstrap { output_dir , dry_run , cargo_only } => { run_bootstrap_mode (cli . verbose , * dry_run , output_dir . as_ref () , * cargo_only , & split_decls_rs :: config_macros :: GLOBAL_CONFIG . lock () . unwrap ()) ? ; } } Ok (()) }