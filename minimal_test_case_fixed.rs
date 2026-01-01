#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

extern crate tracing;
extern crate proc_macro;

// Include wrap_types for basic infrastructure
include!("src/wrap_types.rs");

// Create reg module stub
pub mod reg {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Reg;
    
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct RegKind;
}

include!("src/processed_rustc_abi_rustc_abi_src_callconv.rs");

fn main() {}
