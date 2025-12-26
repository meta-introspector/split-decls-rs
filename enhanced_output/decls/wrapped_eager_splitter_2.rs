// Generated from: ./src/eager_splitter.rs
// Original file: ./src/eager_splitter.rs
// Function: extract_declarations_to_map

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

#[decl_split_decls_rs_eager_splitter]
# [doc = " Extracts declarations from a crate's lib.rs and returns them as a map."] pub fn extract_declarations_to_map (paths : & CratePaths) -> Result < HashMap < String , TokenStream > > { let lib_content = fs :: read_to_string (& paths . lib_rs_path) . context (format ! ("Failed to read {}" , paths . lib_rs_path . display ())) ? ; let syntax_tree : syn :: File = syn :: parse_file (& lib_content) . context ("Failed to parse lib.rs as Rust code") ? ; let mut extracted_decls : HashMap < String , TokenStream > = HashMap :: new () ; let mut item_count = 0 ; for item in & syntax_tree . items { if let Some (decl) = declaration_extractor :: extract_single_declaration (item , item_count) { extracted_decls . insert (decl . name , decl . content) ; item_count += 1 ; } } Ok (extracted_decls) }