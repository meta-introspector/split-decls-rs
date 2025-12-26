// Generated from: ./src/bin/wrap_bin.rs
// Original file: ./src/bin/wrap_bin.rs
// Function: generate_wrapped_main

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

#[decl_split_decls_rs_wrap_bin]
fn generate_wrapped_main (binary_name : & str) -> Result < String > { let wrapped_crate_name = format ! ("wrapped_{}" , binary_name . replace ("-" , "_")) ; Ok (format ! (r#"// Generated wrapped main.rs for {}
// This calls the main function from the wrapped library

use anyhow::Result;

fn main() -> Result<()> {{
    // Import and call the wrapped main function
    {}::main()
}}
"# , binary_name , wrapped_crate_name)) }