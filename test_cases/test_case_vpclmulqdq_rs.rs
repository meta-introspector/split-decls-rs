// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/vpclmulqdq.rs
// Error: expected square brackets
// Problematic line: line 11

use crate::core_arch::x86::__m256i;
use crate::core_arch::x86::__m512i;

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
