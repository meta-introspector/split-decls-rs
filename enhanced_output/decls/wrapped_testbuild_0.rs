// Generated from: ./src/bin/testbuild.rs
// Original file: ./src/bin/testbuild.rs
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

#[decl_split_decls_rs_testbuild]
fn main () -> anyhow :: Result < () > { let generated_cargo_toml : CargoToml = define_root_cargo_toml ! { [package] { mkbuildrs ! () } } ; let toml_string = toml :: to_string_pretty (& generated_cargo_toml) . expect ("Failed to serialize CargoToml to TOML string") ; println ! ("{}" , toml_string) ; Ok (()) }