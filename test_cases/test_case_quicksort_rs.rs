// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/slice/sort/stable/quicksort.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::slice::sort::shared::smallsort::StableSmallSortTypeImpl;
use crate::{intrinsics, ptr};

/// Sorts `v` recursively using quicksort.
/// `scratch.len()` must be at least `max(v.len() - v.len() / 2, SMALL_SORT_GENERAL_SCRATCH_LEN)`
/// otherwise the implementation may abort.
///
