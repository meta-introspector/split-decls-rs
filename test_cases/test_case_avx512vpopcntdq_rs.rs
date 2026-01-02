// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/avx512vpopcntdq.rs
// Error: expected square brackets
// Problematic line: line 19

use crate::intrinsics::simd::{simd_ctpop, simd_select_bitmask};
use crate::mem::transmute;

#[cfg(test)]
use stdarch_test::assert_instr;

/// For each packed 32-bit integer maps the value to the number of logical 1 bits.
