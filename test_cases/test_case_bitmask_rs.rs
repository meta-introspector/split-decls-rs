// MINIMAL TEST CASE for parsing failure in: ../rust/library/portable-simd/crates/core_simd/src/masks/bitmask.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::simd::{LaneCount, Simd, SupportedLaneCount};
use core::marker::PhantomData;

/// A mask where each lane is represented by a single bit.
#[repr(transparent)]
pub(crate) struct Mask<T, const N: usize>(
    <LaneCount<N> as SupportedLaneCount>::BitMask,
