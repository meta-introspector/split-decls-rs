// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/slice/sort/unstable/heapsort.rs
// Error: expected square brackets
// Problematic line: line 5


use crate::{cmp, intrinsics, ptr};

/// Sorts `v` using heapsort, which guarantees *O*(*n* \* log(*n*)) worst-case.
///
/// Never inline this, it sits the main hot-loop in `recurse` and is meant as unlikely algorithmic
/// fallback.
