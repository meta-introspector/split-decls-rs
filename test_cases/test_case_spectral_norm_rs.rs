// MINIMAL TEST CASE for parsing failure in: ../rust/library/portable-simd/crates/core_simd/examples/spectral_norm.rs
// Error: expected square brackets
// Problematic line: line 5


use core_simd::simd::prelude::*;

fn a(i: usize, j: usize) -> f64 {
    ((i + j) * (i + j + 1) / 2 + i + 1) as f64
}

