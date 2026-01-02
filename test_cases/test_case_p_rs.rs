// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/riscv_shared/p.rs
// Error: expected square brackets
// Problematic line: line 5

//!
//! RV64 only part is placed in riscv64 folder.
use crate::arch::asm;
#[cfg(test)]
use stdarch_test::assert_instr;

// FIXME: Currently the P extension is still unratified, so there is no support
