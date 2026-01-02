// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86_64/bswap.rs
// Error: expected square brackets
// Problematic line: line 5


#![allow(clippy::module_name_repetitions)]

#[cfg(test)]
use stdarch_test::assert_instr;

/// Returns an integer with the reversed byte order of x
