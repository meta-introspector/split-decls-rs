// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/slice/sort/shared/smallsort.rs
// Error: expected square brackets
// Problematic line: line 16

// optimize sorting small sub-slices with more sophisticated solutions than
// insertion sort.

/// Using a trait allows us to specialize on `Freeze` which in turn allows us to make safe
/// abstractions.
pub(crate) trait StableSmallSortTypeImpl: Sized {
    /// For which input length <= return value of this function, is it valid to call `small_sort`.
