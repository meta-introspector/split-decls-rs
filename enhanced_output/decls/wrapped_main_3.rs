// Generated from: ./src/main.rs
// Original file: ./src/main.rs
// Function: run_bootstrap_mode

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
fn run_bootstrap_mode (verbose : bool , dry_run : bool , output_dir_override : Option < & PathBuf > , cargo_only : bool , global_config : & SplitDeclsConfig ,) -> Result < () > { if verbose { if dry_run { println ! ("*** Running in DRY-RUN mode. No files will be modified. ***") ; } println ! ("Running bootstrap mode.") ; } let wrapped_workspace_output_dir = output_dir_override . map (| p | p . to_path_buf ()) . unwrap_or_else (| | PathBuf :: from ("output2")) ; let module_not_found_errors = run_wrapped_workspace_mode (verbose , dry_run , output_dir_override , global_config , cargo_only) ? ; let build_workflow = Workflow { name : "Bootstrap Build Stage" . to_string () , description : "Builds the generated code in the output directory." . to_string () , style_influences : vec ! [] , stages : vec ! [split_decls_rs :: goal_parser :: Stage { name : "Build Generated Code" . to_string () , description : format ! ("Runs 'cargo build' in the generated workspace at {}." , wrapped_workspace_output_dir . display ()) , processor_hint : None , inputs : vec ! [] , outputs : vec ! [split_decls_rs :: goal_parser :: Output { name : "stdout" . to_string () , output_type : "string" . to_string () , description : "Standard output of the build command." . to_string () , } , split_decls_rs :: goal_parser :: Output { name : "stderr" . to_string () , output_type : "string" . to_string () , description : "Standard error of the build command." . to_string () , } , split_decls_rs :: goal_parser :: Output { name : "status" . to_string () , output_type : "integer" . to_string () , description : "Exit status code of the build command." . to_string () , } ,] , operation : split_decls_rs :: goal_parser :: Operation :: Shell (split_decls_rs :: goal_parser :: ShellCommandOperation { op_type : "shell" . to_string () , command : "cargo build" . to_string () , working_dir : Some (wrapped_workspace_output_dir . to_string_lossy () . to_string ()) , capture_output : true , error_on_failure : true , } ,) , tasks : vec ! [] , } ,] , } ; let mut workflow_executor = WorkflowExecutor :: new (verbose , dry_run , global_config . clone ()) ; workflow_executor . execute (& build_workflow) ? ; if verbose { if let Some (status_value) = workflow_executor . get_context_value ("status") { if let Some (status) = status_value . as_integer () { println ! ("Build command exited with status: {}" , status) ; if status != 0 { println ! ("Build failed. See 'stderr' in context for details.") ; } } } } if ! module_not_found_errors . is_empty () { warn ! ("\n--- Module Not Found Summary ---") ; for error_report in module_not_found_errors { warn ! ("  Crate: '{}', Module: '{}', Message: '{}', Generated File: '{}'" , error_report . crate_name , error_report . module_name , error_report . error_message , error_report . generated_file_path . display ()) ; } warn ! ("------------------------------") ; } Ok (()) }