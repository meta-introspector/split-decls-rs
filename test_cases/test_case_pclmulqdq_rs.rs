// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/pclmulqdq.rs
// Error: expected square brackets
// Problematic line: line 10


use crate::core_arch::x86::__m128i;

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
