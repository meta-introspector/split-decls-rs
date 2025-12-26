// Generated from: ./src/bin/extract_crate.rs
// Original file: ./src/bin/extract_crate.rs
// Function: parse_dependencies

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

#[decl_split_decls_rs_extract_crate]
# [doc = " Parse dependencies from Cargo.toml"] fn parse_dependencies (cargo_toml : & Path) -> Result < (HashSet < String > , HashSet < String >) > { let content = fs :: read_to_string (cargo_toml) ? ; let toml : Value = toml :: from_str (& content) ? ; let mut deps = HashSet :: new () ; let mut dev_deps = HashSet :: new () ; if let Some (dependencies) = toml . get ("dependencies") . and_then (| v | v . as_table ()) { for (name , _) in dependencies { deps . insert (name . clone ()) ; } } if let Some (dev_dependencies) = toml . get ("dev-dependencies") . and_then (| v | v . as_table ()) { for (name , _) in dev_dependencies { dev_deps . insert (name . clone ()) ; } } Ok ((deps , dev_deps)) }