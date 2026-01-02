// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/overflow_panic.rs
// Error: expected square brackets
// Problematic line: line 5

//!
//! In particular, these are used by the `strict_` methods on integers.

#[cold]
#[track_caller]
pub(super) const fn add() -> ! {
    panic!("attempt to add with overflow")
