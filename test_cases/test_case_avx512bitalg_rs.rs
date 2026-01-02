// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/avx512bitalg.rs
// Error: expected square brackets
// Problematic line: line 26

use crate::intrinsics::simd::{simd_ctpop, simd_select_bitmask};
use crate::mem::transmute;

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
