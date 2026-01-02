// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/nvptx/packed.rs
// Error: expected square brackets
// Problematic line: line 9


use crate::intrinsics::simd::*;

#[allow(improper_ctypes)]
unsafe extern "C" {
    #[link_name = "llvm.minimum.v2f16"]
    fn llvm_f16x2_minimum(a: f16x2, b: f16x2) -> f16x2;
