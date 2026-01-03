#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
#![feature(no_core)]
#![feature(generic_atomic)]
#![feature(alloc_error_handler)]

// External rustc compiler crates
extern crate rustc_ast;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_hir;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_session;

// Include generated rustc modules
include!(concat!(env!("OUT_DIR"), "/rustc_includes.rs"));

// Re-export for convenience
pub use rustc_complete::*;

// Main library functionality
pub fn initialize_genesis() {
    println!("🌟 Split-Decls Genesis System Initialized");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_initialization() {
        initialize_genesis();
    }
}
