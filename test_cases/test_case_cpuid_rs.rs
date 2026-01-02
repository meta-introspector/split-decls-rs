// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/cpuid.rs
// Error: expected square brackets
// Problematic line: line 5

#![allow(clippy::module_name_repetitions)]

use crate::arch::asm;
#[cfg(test)]
use stdarch_test::assert_instr;

/// Result of the `cpuid` instruction.
