// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/str/count.rs
// Error: expected square brackets
// Problematic line: line 26

const USIZE_SIZE: usize = size_of::<usize>();
const UNROLL_INNER: usize = 4;

#[inline]
pub(super) fn count_chars(s: &str) -> usize {
    if cfg!(feature = "optimize_for_size") || s.len() < USIZE_SIZE * UNROLL_INNER {
        // Avoid entering the optimized implementation for strings where the
