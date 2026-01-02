// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/sse2.rs
// Error: expected square brackets
// Problematic line: line 3

//! Streaming SIMD Extensions 2 (SSE2)

#[cfg(test)]
use stdarch_test::assert_instr;

use crate::{
