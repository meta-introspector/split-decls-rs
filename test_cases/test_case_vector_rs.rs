// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/s390x/vector.rs
// Error: expected square brackets
// Problematic line: line 12


use crate::{core_arch::simd::*, intrinsics::simd::*, mem::MaybeUninit, mem::transmute};

#[cfg(test)]
use stdarch_test::assert_instr;

use super::macros::*;
