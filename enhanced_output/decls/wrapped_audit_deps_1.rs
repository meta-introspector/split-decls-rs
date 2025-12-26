// Generated from: ./src/bin/audit_deps.rs
// Original file: ./src/bin/audit_deps.rs
// Function: audit_crate_dependencies

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

#[decl_split_decls_rs_audit_deps]
fn audit_crate_dependencies (cargo_toml_path : & Path , crate_name : & str , workspace_deps : & HashMap < String , Value > , available_submodules : & HashMap < String , PathBuf > , issues : & mut Vec < DependencyIssue > ,) -> Result < () > { let content = fs :: read_to_string (cargo_toml_path) ? ; let toml : Value = toml :: from_str (& content) ? ; for dep_section in ["dependencies" , "dev-dependencies" , "build-dependencies"] { if let Some (deps) = toml . get (dep_section) { if let Value :: Table (deps_table) = deps { for (dep_name , dep_spec) in deps_table { if let Value :: Table (spec_table) = dep_spec { if spec_table . contains_key ("workspace") { if ! workspace_deps . contains_key (dep_name) { issues . push (DependencyIssue { crate_name : crate_name . to_string () , dep_name : dep_name . clone () , issue_type : IssueType :: MissingFromWorkspace , current_spec : "workspace = true" . to_string () , suggested_fix : available_submodules . get (dep_name) . map (| _ | format ! ("path = \"../submodules/{}\"" , dep_name)) , }) ; } } else if ! spec_table . contains_key ("path") { if available_submodules . contains_key (dep_name) { issues . push (DependencyIssue { crate_name : crate_name . to_string () , dep_name : dep_name . clone () , issue_type : IssueType :: MissingSubmodule , current_spec : format ! ("{:?}" , dep_spec) , suggested_fix : Some (format ! ("path = \"../submodules/{}\"" , dep_name)) , }) ; } } } } } } } Ok (()) }