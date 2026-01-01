
#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

include!("wrap_types.rs");

pub mod rustc_macros {
    pub struct HashStable_Generic;
}


include!("processed_.._rust_compiler_rustc_abi_src_callconv_reg.rs");
