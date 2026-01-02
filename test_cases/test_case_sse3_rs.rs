// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/sse3.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::core_arch::{simd::*, x86::*};
use crate::intrinsics::simd::*;

#[cfg(test)]
use stdarch_test::assert_instr;

/// Alternatively add and subtract packed single-precision (32-bit)
