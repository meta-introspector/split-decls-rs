// Generated from: ./src/bin/extract_crate.rs
// Original file: ./src/bin/extract_crate.rs
// Function: create_standalone_workspace

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

#[decl_split_decls_rs_extract_crate]
# [doc = " Create standalone workspace with extracted crates"] fn create_standalone_workspace (output_path : & Path , target_crate : & CrateInfo , all_crates : & HashSet < String > , dependency_graph : & HashMap < String , CrateInfo > , verbose : bool ,) -> Result < () > { fs :: create_dir_all (output_path) ? ; for crate_name in all_crates { if let Some (crate_info) = dependency_graph . get (crate_name) { let dest_path = output_path . join (& crate_info . name) ; if verbose { println ! ("  📁 Copying: {} → {}" , crate_info . path . display () , dest_path . display ()) ; } copy_dir_recursive (& crate_info . path , & dest_path) ? ; } } let workspace_toml = generate_workspace_toml (target_crate , all_crates , dependency_graph) ? ; fs :: write (output_path . join ("Cargo.toml") , workspace_toml) ? ; let readme = generate_extracted_readme (target_crate , all_crates) ? ; fs :: write (output_path . join ("README.md") , readme) ? ; Ok (()) }