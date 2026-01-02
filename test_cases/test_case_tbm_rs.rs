// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86_64/tbm.rs
// Error: expected square brackets
// Problematic line: line 13

//! [wikipedia_bmi]:
//! https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#ABM_.28Advanced_Bit_Manipulation.29

#[cfg(test)]
use stdarch_test::assert_instr;

unsafe extern "C" {
