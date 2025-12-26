// Generated from: ./src/bin/macro_runner.rs
// Original file: ./src/bin/macro_runner.rs
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

#[decl_split_decls_rs_macro_runner]
fn main () -> Result < () > { let args = Args :: parse () ; println ! ("🎯 Macro Runner: Executing {} with args: {:?}" , args . macro_name , args . args) ; let result = match args . macro_name . as_str () { "wrap_single_crate" => { wrap_single_crate_macro ! (args . args) } , "bootstrap" => { bootstrap_macro ! (args . args) } , "add_wrapped_crate" => { println ! ("➕ Executing add_wrapped_crate via macro") ; println ! ("🔧 Would add wrapped crate with args: {:?}" , args . args) ; Ok (()) } , "dwim" => { println ! ("🧠 Executing DWIM system") ; println ! ("🔧 DWIM would figure out what to do with: {:?}" , args . args) ; Ok (()) } , _ => { println ! ("❌ Unknown macro: {}" , args . macro_name) ; println ! ("Available macros: wrap_single_crate, bootstrap, add_wrapped_crate, dwim") ; Err (anyhow :: anyhow ! ("Unknown macro: {}" , args . macro_name)) } } ; result ? ; println ! ("✅ Macro execution completed") ; Ok (()) }