// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/arm/simd32.rs
// Error: expected square brackets
// Problematic line: line 65

//! - \[x\] __smusd
//! - \[x\] __smusdx

#[cfg(test)]
use stdarch_test::assert_instr;

use crate::mem::transmute;
