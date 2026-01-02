// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/fxsr.rs
// Error: expected square brackets
// Problematic line: line 3

//! FXSR floating-point context fast save and restore.

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
