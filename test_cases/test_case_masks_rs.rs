// MINIMAL TEST CASE for parsing failure in: ../rust/library/portable-simd/crates/core_simd/src/masks.rs
// Error: expected square brackets
// Problematic line: line 5

//! Types representing
#![allow(non_camel_case_types)]

#[cfg_attr(
    not(all(target_arch = "x86_64", target_feature = "avx512f")),
    path = "masks/full_masks.rs"
)]
