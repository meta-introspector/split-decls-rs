// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/example/neon.rs
// Error: expected square brackets
// Problematic line: line 5


#![feature(portable_simd)]

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "aarch64")]
use std::mem::transmute;
