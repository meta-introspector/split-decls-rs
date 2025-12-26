// Generated from: ./src/bin/output2-wrapper.rs
// Original file: ./src/bin/output2-wrapper.rs
// Function: inspect_wrapped_crate

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

#[decl_split_decls_rs_output2-wrapper]
fn inspect_wrapped_crate (crate_name : & str , details : bool) -> anyhow :: Result < () > { let wrapped_path = format ! ("output2/wrapped-{}" , crate_name) ; let decls_path = format ! ("{}/src/decls" , wrapped_path) ; if let Ok (entries) = std :: fs :: read_dir (& decls_path) { let decl_files : Vec < _ > = entries . flatten () . collect () ; println ! ("📋 Found {} declarations in {}" , decl_files . len () , wrapped_path) ; if details { for entry in decl_files . iter () . take (10) { let file_name_string = entry . file_name () . to_string_lossy () . to_string () ; if let Ok (metadata) = entry . metadata () { println ! ("  {} ({} bytes)" , file_name_string , metadata . len ()) ; } } } } else { println ! ("❌ Wrapped crate not found: {}" , wrapped_path) ; } Ok (()) }