// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/powerpc/altivec.rs
// Error: expected square brackets
// Problematic line: line 18


use crate::{core_arch::simd::*, intrinsics::simd::*, mem, mem::transmute};

#[cfg(test)]
use stdarch_test::assert_instr;

use super::macros::*;
