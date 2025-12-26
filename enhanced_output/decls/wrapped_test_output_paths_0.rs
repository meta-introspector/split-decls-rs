// Generated from: ./src/bin/test_output_paths.rs
// Original file: ./src/bin/test_output_paths.rs
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

#[decl_split_decls_rs_test_output_paths]
fn main () { println ! ("Testing CratePaths generation with output directory...") ; let crate_path = PathBuf :: from ("test_crate") ; let paths = setup_crate_paths (& crate_path) . unwrap () ; println ! ("Crate name: {}" , paths . crate_name) ; println ! ("Crate path: {}" , paths . crate_path . display ()) ; println ! ("Lib.rs path: {}" , paths . lib_rs_path . display ()) ; println ! ("Lib.rs path: {}" , paths . lib_rs_path . display ()) ; println ! ("Decls output dir: {}" , paths . decls_output_dir . display ()) ; let expected_output = PathBuf :: from ("output") . join ("test_crate") . join ("src") . join ("decls") ; assert_eq ! (paths . decls_output_dir , expected_output) ; println ! ("✅ Output directory path is correct: {}" , expected_output . display ()) ; println ! ("✅ Test passed!") ; }