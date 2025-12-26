// Generated from: ./src/bin/extract_crate.rs
// Original file: ./src/bin/extract_crate.rs
// Function: discover_dependencies

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
# [doc = " Recursively discover all dependencies"] fn discover_dependencies (output2_path : & Path , crate_info : & CrateInfo , dependency_graph : & mut HashMap < String , CrateInfo > , all_crates : & mut HashSet < String > , max_depth : usize , include_dev_deps : bool , verbose : bool ,) -> Result < () > { if max_depth == 0 || all_crates . contains (& crate_info . name) { return Ok (()) ; } all_crates . insert (crate_info . name . clone ()) ; dependency_graph . insert (crate_info . name . clone () , crate_info . clone ()) ; if verbose { println ! ("  📋 Processing: {} ({} deps)" , crate_info . name , crate_info . dependencies . len ()) ; } for dep_name in & crate_info . dependencies { if let Ok (dep_crate) = find_wrapped_crate (output2_path , dep_name) { discover_dependencies (output2_path , & dep_crate , dependency_graph , all_crates , max_depth - 1 , include_dev_deps , verbose ,) ? ; } } if include_dev_deps { for dep_name in & crate_info . dev_dependencies { if let Ok (dep_crate) = find_wrapped_crate (output2_path , dep_name) { discover_dependencies (output2_path , & dep_crate , dependency_graph , all_crates , max_depth - 1 , include_dev_deps , verbose ,) ? ; } } } Ok (()) }