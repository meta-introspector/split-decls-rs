// Generated from: ./src/bin/wrap_bin.rs
// Original file: ./src/bin/wrap_bin.rs
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

#[decl_split_decls_rs_wrap_bin]
fn main () -> Result < () > { let args = Args :: parse () ; if args . verbose { println ! ("Generating wrapped binary for: {}" , args . binary_name) ; println ! ("Output directory: {}" , args . output_dir . display ()) ; } let src_dir = args . output_dir . join ("src") ; fs :: create_dir_all (& src_dir) ? ; let main_content = generate_wrapped_main (& args . binary_name) ? ; let main_path = src_dir . join ("main.rs") ; fs :: write (& main_path , main_content) ? ; let cargo_content = generate_wrapped_cargo_toml (& args . binary_name) ? ; let cargo_path = args . output_dir . join ("Cargo.toml") ; fs :: write (& cargo_path , cargo_content) ? ; if args . verbose { println ! ("✅ Generated wrapped binary files:") ; println ! ("  - {}" , main_path . display ()) ; println ! ("  - {}" , cargo_path . display ()) ; } Ok (()) }