#![recursion_limit = "512"]
#![allow(internal_features)]
#![allow(unused)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(panic_backtrace_config)]
#![feature(panic_update_hook)]
#![feature(rustdoc_internals)]
#![feature(try_blocks)]

// External crates needed for rustc
extern crate rustc_driver_impl;
extern crate rustc_data_structures;
extern crate rustc_session;
extern crate rustc_errors;
extern crate rustc_span;
extern crate rustc_ast;
extern crate rustc_middle;
extern crate rustc_hir;
extern crate rustc_codegen_ssa;

// Include the actual rustc main
use rustc_driver_impl::main as rustc_main;

fn main() -> ! {
    rustc_main()
}
