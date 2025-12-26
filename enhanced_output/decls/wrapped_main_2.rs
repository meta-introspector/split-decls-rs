// Generated from: ./src/main.rs
// Original file: ./src/main.rs
// Function: run_execute_goal_workflow_mode

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
fn run_execute_goal_workflow_mode (verbose : bool , dry_run : bool , goal_file : & PathBuf , global_config : & SplitDeclsConfig ,) -> Result < () > { if verbose { if dry_run { println ! ("*** Running in DRY-RUN mode. No files will be modified. ***") ; } println ! ("Executing workflow from: {}" , goal_file . display ()) ; } let goal_config = GoalConfig :: load_from_file (goal_file) . context (format ! ("Failed to load goal file from {}" , goal_file . display ())) ? ; let mut workflow_executor = WorkflowExecutor :: new (verbose , dry_run , global_config . clone ()) ; workflow_executor . execute (& goal_config . workflow) ? ; Ok (()) }