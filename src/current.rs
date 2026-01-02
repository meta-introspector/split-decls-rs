#![recursion_limit = "512"]
#![allow(internal_features)]
#![allow(unused)]
#![allow(rustc::untranslatable_diagnostic)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(panic_backtrace_config)]
#![feature(panic_update_hook)]
#![feature(rustdoc_internals)]
#![feature(try_blocks)]

extern crate rustc_passes;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate rustc_resolve;
extern crate rustc_middle;
extern crate rustc_metadata;
extern crate rustc_const_eval;
extern crate rustc_span;
extern crate rustc_hir_typeck;
extern crate rustc_traits;
extern crate rustc_lint;
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_borrowck;
extern crate rustc_hir_analysis;
extern crate rustc_mir_build;
extern crate rustc_driver;
extern crate rustc_driver_impl;
extern crate rustc_expand;
extern crate rustc_mir_dataflow;
extern crate rustc_codegen_ssa;
extern crate rustc_parse;
extern crate rustc_mir_transform;
extern crate rustc_serialize;
extern crate rustc_index;
extern crate rustc_macros;
extern crate rustc_builtin_macros;
extern crate rustc_errors;
extern crate rustc_privacy;
extern crate rustc_session;
extern crate rustc_data_structures;
extern crate rustc_interface;
extern crate rustc_infer;

// Custom macro to include processed rustc files
macro_rules! include_rustc {
    ($crate_name:ident, $file:ident) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/submodules/rust/compiler/", stringify!($crate_name), "/src/", stringify!($file), ".rs"));
    };
    ($crate_name:ident) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/submodules/rust/compiler/", stringify!($crate_name), "/src/lib.rs"));
    };
}

// === TARGET: rustc_driver::main ===
fn main() {
    println!("Executing target: rustc_driver::main");
}
