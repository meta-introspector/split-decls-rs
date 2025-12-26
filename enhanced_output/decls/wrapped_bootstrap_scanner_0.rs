// Generated from: ./src/bin/bootstrap_scanner.rs
// Original file: ./src/bin/bootstrap_scanner.rs
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

#[decl_split_decls_rs_bootstrap_scanner]
fn main () -> Result < () > { println ! ("🚀 Bootstrap Duplicate Code Scanner") ; println ! ("Using loaded macros as search keys...\n") ; let mut scanner = DuplicateScanner :: new () ? ; scanner . scan_wrapped_output ("output2") ? ; scanner . generate_repl_commands () ? ; println ! ("\n✨ Scan complete! Use the generated REPL commands to explore duplicates.") ; Ok (()) }