// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/slice/sort/stable/tiny.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::ptr;
use crate::slice::sort::stable::merge;

/// Tiny recursive top-down merge sort optimized for binary size. It has no adaptiveness whatsoever,
/// no run detection, etc.
#[inline(always)]
pub fn mergesort<T, F: FnMut(&T, &T) -> bool>(
