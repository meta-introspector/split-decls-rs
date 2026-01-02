// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/mips/msa.rs
// Error: expected square brackets
// Problematic line: line 8

//!
//! [msa_ref]: http://cdn2.imgtec.com/documentation/MD00866-2B-MSA32-AFP-01.12.pdf

#[cfg(test)]
use stdarch_test::assert_instr;

use crate::mem;
