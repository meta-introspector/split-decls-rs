// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/f16c.rs
// Error: expected square brackets
// Problematic line: line 7


use crate::core_arch::{simd::*, x86::*};

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
