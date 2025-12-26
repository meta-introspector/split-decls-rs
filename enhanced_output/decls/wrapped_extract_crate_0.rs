// Generated from: ./src/bin/extract_crate.rs
// Original file: ./src/bin/extract_crate.rs
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

#[decl_split_decls_rs_extract_crate]
fn main () -> Result < () > { let args = Args :: parse () ; println ! ("🔍 Extracting {} with dependencies to {}" , args . crate_name , args . output . display ()) ; let output2_path = Path :: new ("output2") ; let target_crate = find_wrapped_crate (& output2_path , & args . crate_name) ? ; if args . verbose { println ! ("📦 Found target crate: {}" , target_crate . path . display ()) ; } let mut dependency_graph = HashMap :: new () ; let mut all_crates = HashSet :: new () ; discover_dependencies (& output2_path , & target_crate , & mut dependency_graph , & mut all_crates , args . max_depth , args . include_dev_deps , args . verbose ,) ? ; println ! ("📊 Discovered {} total crates (including dependencies)" , all_crates . len ()) ; create_standalone_workspace (& args . output , & target_crate , & all_crates , & dependency_graph , args . verbose ,) ? ; println ! ("✅ Standalone workspace created at {}" , args . output . display ()) ; println ! ("   Main crate: {}" , args . crate_name) ; println ! ("   Total crates: {}" , all_crates . len ()) ; println ! ("   Ready for: cd {} && cargo build" , args . output . display ()) ; Ok (()) }