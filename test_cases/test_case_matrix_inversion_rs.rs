// MINIMAL TEST CASE for parsing failure in: ../rust/library/portable-simd/crates/core_simd/examples/matrix_inversion.rs
// Error: expected square brackets
// Problematic line: line 8

use core_simd::simd::prelude::*;

// Gotta define our own 4x4 matrix since Rust doesn't ship multidim arrays yet :^)
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix4x4([[f32; 4]; 4]);

#[allow(clippy::too_many_lines)]
