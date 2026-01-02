// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/slice/sort/stable/drift.rs
// Error: expected square brackets
// Problematic line: line 11

use crate::slice::sort::stable::quicksort::quicksort;
use crate::{cmp, intrinsics};

/// Sorts `v` based on comparison function `is_less`. If `eager_sort` is true,
/// it will only do small-sorts and physical merges, ensuring O(N * log(N))
/// worst-case complexity. `scratch.len()` must be at least
/// `max(v.len() - v.len() / 2, SMALL_SORT_GENERAL_SCRATCH_LEN)` otherwise the implementation may abort.
