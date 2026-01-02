// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/wasm32/simd128.rs
// Error: expected square brackets
// Problematic line: line 11


use crate::{core_arch::simd, intrinsics::simd::*, marker::Sized, mem, ptr};

#[cfg(test)]
use stdarch_test::assert_instr;

types! {
