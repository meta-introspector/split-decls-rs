// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/ssse3.rs
// Error: expected square brackets
// Problematic line: line 3

//! Supplemental Streaming SIMD Extensions 3 (SSSE3)

use crate::{
    core_arch::{simd::*, x86::*},
    intrinsics::simd::*,
};
