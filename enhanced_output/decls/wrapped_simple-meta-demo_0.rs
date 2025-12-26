// Generated from: ./src/bin/simple-meta-demo.rs
// Original file: ./src/bin/simple-meta-demo.rs
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

#[decl_split_decls_rs_simple-meta-demo]
fn main () { println ! ("🚀 8-LAYER META PATTERN SYSTEM DEMO") ; println ! ("Enum-driven parameter generation with no hardcoded strings\n") ; let mut counter = MetaPatternCounter :: new () ; let patterns = [SynLangPatterns :: File , SynLangPatterns :: ItemFn , SynLangPatterns :: Expr , SynLangPatterns :: Type ,] ; println ! ("📊 PATTERN METADATA GENERATION:") ; for pattern in & patterns { let name = pattern . as_str () ; let method = pattern . visit_method () ; println ! ("  {} -> {}" , name , method) ; counter . increment (name) ; } println ! ("\n🔗 LAYER FLOW DEMONSTRATION:") ; println ! ("Layer 1: Enum variants defined") ; println ! ("Layer 2: mkmeta! generates as_str() and visit_method()") ; println ! ("Layer 3: Metadata applied to enum") ; println ! ("Layer 4-8: Parameters flow through macro layers") ; counter . report () ; println ! ("\n✅ NO HARDCODED STRINGS - ALL GENERATED FROM ENUM!") ; }