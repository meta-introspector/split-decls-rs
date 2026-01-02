// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/arm_shared/neon/generated.rs
// Error: expected square brackets
// Problematic line: line 10

// ```
#![allow(improper_ctypes)]

#[cfg(test)]
use stdarch_test::assert_instr;

use super::*;
