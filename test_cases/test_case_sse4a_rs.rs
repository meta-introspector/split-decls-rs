// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/sse4a.rs
// Error: expected square brackets
// Problematic line: line 5


use crate::core_arch::{simd::*, x86::*};

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
