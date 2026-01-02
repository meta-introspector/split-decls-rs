// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/slice/sort/stable/merge.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::mem::MaybeUninit;
use crate::{cmp, ptr};

/// Merges non-decreasing runs `v[..mid]` and `v[mid..]` using `scratch` as
/// temporary storage, and stores the result into `v[..]`.
pub fn merge<T, F: FnMut(&T, &T) -> bool>(
    v: &mut [T],
