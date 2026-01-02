// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_gcc/example/std_example.rs
// Error: expected square brackets
// Problematic line: line 4

#![allow(internal_features)]
#![feature(core_intrinsics, coroutines, coroutine_trait, stmt_expr_attributes)]

#[cfg(feature="master")]
#[cfg(target_arch="x86_64")]
use std::arch::x86_64::*;
use std::io::Write;
