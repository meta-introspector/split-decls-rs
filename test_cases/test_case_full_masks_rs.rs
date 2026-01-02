// MINIMAL TEST CASE for parsing failure in: ../rust/library/portable-simd/crates/core_simd/src/masks/full_masks.rs
// Error: expected square brackets
// Problematic line: line 5


use crate::simd::{LaneCount, MaskElement, Simd, SupportedLaneCount};

#[repr(transparent)]
pub(crate) struct Mask<T, const N: usize>(Simd<T, N>)
where
    T: MaskElement,
