// Generated from: ./src/bin/extract_crate.rs
// Original file: ./src/bin/extract_crate.rs
// Function: find_wrapped_crate

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
# [doc = " Find wrapped crate by name in output2"] fn find_wrapped_crate (output2_path : & Path , crate_name : & str) -> Result < CrateInfo > { let wrapped_name = format ! ("wrapped-{}" , crate_name) ; let crate_path = output2_path . join (& wrapped_name) ; if ! crate_path . exists () { return Err (anyhow :: anyhow ! ("Wrapped crate not found: {}" , wrapped_name)) ; } let cargo_toml = crate_path . join ("Cargo.toml") ; let (deps , dev_deps) = parse_dependencies (& cargo_toml) ? ; Ok (CrateInfo { name : wrapped_name , path : crate_path , dependencies : deps , dev_dependencies : dev_deps , }) }