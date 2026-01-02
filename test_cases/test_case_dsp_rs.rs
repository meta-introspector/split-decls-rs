// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/arm/dsp.rs
// Error: expected square brackets
// Problematic line: line 23

//! - \[x\] __smlawb
//! - \[x\] __smlawt

#[cfg(test)]
use stdarch_test::assert_instr;

unsafe extern "unadjusted" {
