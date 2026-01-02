// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/rdtsc.rs
// Error: expected square brackets
// Problematic line: line 3

//! RDTSC instructions.

#[cfg(test)]
use stdarch_test::assert_instr;

/// Reads the current value of the processor’s time-stamp counter.
