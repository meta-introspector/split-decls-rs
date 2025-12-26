// Generated from: ./src/bin/stateful_repl.rs
// Original file: ./src/bin/stateful_repl.rs
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

#[decl_split_decls_rs_stateful_repl]
fn main () -> Result < () > { println ! ("🚀 Stateful REPL - RDF Blob State System") ; println ! ("Commands: set <var>=<value>, get <var>, save, load, export <type>, quit") ; println ! ("State file: {}\n" , STATE_FILE) ; let mut state = ReplState :: load_from_file () ? ; loop { print ! ("repl> ") ; io :: stdout () . flush () ? ; let mut input = String :: new () ; io :: stdin () . read_line (& mut input) ? ; let input = input . trim () ; if input . is_empty () { continue ; } state . history . push (input . to_string ()) ; match execute_command (& mut state , input) { Ok (should_continue) => { if ! should_continue { break ; } } Err (e) => { println ! ("❌ Error: {}" , e) ; } } } state . save_to_file () ? ; println ! ("👋 Goodbye!") ; Ok (()) }