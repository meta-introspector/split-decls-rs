
#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

extern crate tracing;
extern crate synstructure;
extern crate proc_macro;

// Include wrap_types for basic infrastructure
include!("wrap_types.rs");

include!("processed_rustc_abi_rustc_abi_src_callconv.rs");

include!("processed_rustc_abi_rustc_abi_src_callconv.rs");
