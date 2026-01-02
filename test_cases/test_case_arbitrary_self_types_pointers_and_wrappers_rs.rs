// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_gcc/example/arbitrary_self_types_pointers_and_wrappers.rs
// Error: expected square brackets
// Problematic line: line 7

#![feature(rustc_attrs)]
#![allow(internal_features)]

use std::{
    ops::{Deref, CoerceUnsized, DispatchFromDyn},
    marker::Unsize,
};
