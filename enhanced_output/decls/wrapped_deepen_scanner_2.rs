// Generated from: ./src/bin/deepen_scanner.rs
// Original file: ./src/bin/deepen_scanner.rs
// Function: scan_output2_blocks

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

#[decl_split_decls_rs_deepen_scanner]
fn scan_output2_blocks () -> Result < HashMap < String , String > > { let mut blocks = HashMap :: new () ; if Path :: new ("output2") . exists () { for entry in fs :: read_dir ("output2") ? { let entry = entry ? ; if entry . path () . extension () . map_or (false , | ext | ext == "rs") { let content = fs :: read_to_string (entry . path ()) ? ; let filename = entry . file_name () . to_string_lossy () . to_string () ; blocks . insert (filename , content) ; } } } Ok (blocks) }