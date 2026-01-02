// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/avx512bf16.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::core_arch::{simd::*, x86::*};
use crate::intrinsics::simd::*;

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
