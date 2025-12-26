// Generated from: ./src/bin/regen_cargo_v2.rs
// Original file: ./src/bin/regen_cargo_v2.rs
// Function: test_build_crates

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

#[decl_split_decls_rs_regen_cargo_v2]
fn test_build_crates (crates : & [PathBuf] , verbose : bool) -> Result < () > { if verbose { println ! ("Building updated crates...") ; } let mut error_count = 0 ; for crate_path in crates { let output = std :: process :: Command :: new ("cargo") . args (& ["check" , "--quiet"]) . current_dir (crate_path) . output () ; match output { Ok (result) if ! result . status . success () => { error_count += 1 ; let crate_name = crate_path . file_name () . unwrap () . to_str () . unwrap () ; println ! ("❌ {}: {}" , crate_name , String :: from_utf8_lossy (& result . stderr) . lines () . next () . unwrap_or ("Build failed")) ; } Err (e) => { error_count += 1 ; let crate_name = crate_path . file_name () . unwrap () . to_str () . unwrap () ; println ! ("❌ {}: {}" , crate_name , e) ; } _ => { } } } if error_count > 0 { println ! ("❌ {} crates failed to build" , error_count) ; std :: process :: exit (101) ; } else if verbose { println ! ("✅ All crates built successfully") ; } Ok (()) }