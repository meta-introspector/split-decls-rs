// Generated from: ./src/bin/test_splitter.rs
// Original file: ./src/bin/test_splitter.rs
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

#[decl_split_decls_rs_test_splitter]
fn main () -> Result < () > { let test_crate_path = PathBuf :: from ("/mnt/data1/nix/vendor/rust/cargo2nix/unimacro_derive") ; let mut config = split_decls_rs :: config_macros :: GLOBAL_CONFIG . lock () . unwrap () . clone () ; config . wrapping = mkwrapping ! () ; println ! ("Running decl splitter on unimacro_derive crate: {}" , test_crate_path . display ()) ; process_crate (& test_crate_path , & config , false) ? ; println ! ("Decl splitting completed successfully!") ; Ok (()) }