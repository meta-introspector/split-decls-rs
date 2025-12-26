// Generated from: ./src/bin/wrap_bin.rs
// Original file: ./src/bin/wrap_bin.rs
// Function: generate_wrapped_cargo_toml

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
fn generate_wrapped_cargo_toml (binary_name : & str) -> Result < String > { let wrapped_crate_name = format ! ("wrapped_{}" , binary_name . replace ("-" , "_")) ; Ok (format ! (r#"[package]
name = "{}-wrapped"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "{}"
path = "src/main.rs"

[dependencies]
{} = {{ path = "./{}" }}
anyhow = "1.0"
"# , binary_name , binary_name , wrapped_crate_name , wrapped_crate_name)) }