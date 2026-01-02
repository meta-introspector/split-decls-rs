// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/arm/sat.rs
// Error: expected square brackets
// Problematic line: line 5

//!
//! - Section 8.4 "Saturating intrinsics"

#[cfg(test)]
use stdarch_test::assert_instr;

/// Saturates a 32-bit signed integer to a signed integer with a given
