// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/gfni.rs
// Error: expected square brackets
// Problematic line: line 22

use crate::intrinsics::simd::simd_select_bitmask;
use crate::mem::transmute;

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
