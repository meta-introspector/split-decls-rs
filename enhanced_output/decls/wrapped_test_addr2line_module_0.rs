// Generated from: ./src/bin/test_addr2line_module.rs
// Original file: ./src/bin/test_addr2line_module.rs
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

#[decl_split_decls_rs_test_addr2line_module]
# [doc = " Test framework to exercise wrapped addr2line items directly"] fn main () -> Result < () > { println ! ("🔥 Exercising wrapped addr2line module items") ; let results = test_wrapped_addr2line_items () ? ; println ! ("📊 Exercise Results:") ; for (i , result) in results . iter () . enumerate () { println ! ("  {}. ✅ {}" , i + 1 , result) ; } println ! ("🎉 Successfully exercised {} wrapped addr2line items!" , results . len ()) ; Ok (()) }