// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/powerpc64/vsx.rs
// Error: expected square brackets
// Problematic line: line 14

use crate::core_arch::powerpc::macros::*;
use crate::core_arch::powerpc::*;

#[cfg(test)]
use stdarch_test::assert_instr;

use crate::mem::transmute;
