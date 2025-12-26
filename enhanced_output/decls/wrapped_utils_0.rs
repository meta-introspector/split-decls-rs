// Generated from: ./src/wrapped_workspace_handlers/utils.rs
// Original file: ./src/wrapped_workspace_handlers/utils.rs
// Function: collect_and_format_workspace_dependencies

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

#[decl_split_decls_rs_utils]
pub fn collect_and_format_workspace_dependencies (global_config : & SplitDeclsConfig , _output_dir : & Path , _scan_root : & Path ,) -> Result < HashMap < String , Value > > { let mut workspace_deps = HashMap :: new () ; eprintln ! ("DEBUG: global_config.workspace_dependencies: {:?}" , global_config . workspace_dependencies) ; eprintln ! ("DEBUG: global_config.workspace_dependency_overrides: {:?}" , global_config . workspace_dependency_overrides) ; for crate_name in & global_config . wrapping . crates { let mut dep_table = Table :: new () ; dep_table . insert ("path" . to_string () , Value :: String (format ! ("wrapped-{}" , crate_name))) ; workspace_deps . insert (crate_name . clone () , Value :: Table (dep_table)) ; } for (dep_name , dep_value) in global_config . workspace_dependencies . iter () { if ! global_config . wrapping . crates . contains (dep_name) { workspace_deps . insert (dep_name . clone () , dep_value . clone ()) ; } } for (dep_name , override_value) in & global_config . workspace_dependency_overrides { if ! workspace_deps . contains_key (dep_name) { workspace_deps . insert (dep_name . clone () , override_value . clone ()) ; } } eprintln ! ("DEBUG: Final workspace_deps: {:?}" , workspace_deps) ; Ok (workspace_deps) }