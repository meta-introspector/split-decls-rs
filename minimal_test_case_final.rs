#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

// Minimal stubs without external dependencies
pub mod wrap_types {
    pub struct Star;
}

// Create reg module stub
pub mod reg {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Reg;
    
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct RegKind;
}

include!("src/processed_rustc_abi_rustc_abi_src_callconv.rs");

fn main() {
    println!("✅ Compilation successful!");
}
