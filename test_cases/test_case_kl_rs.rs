// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/kl.rs
// Error: expected square brackets
// Problematic line: line 8

use crate::core_arch::x86::__m128i;
use crate::ptr;

#[cfg(test)]
use stdarch_test::assert_instr;

#[repr(C, packed)]
