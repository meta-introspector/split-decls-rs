// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/range/legacy.rs
// Error: expected square brackets
// Problematic line: line 9

//!
//! The types here are equivalent to those in [`core::ops`].

#[doc(inline)]
pub use crate::ops::{Range, RangeFrom, RangeInclusive, RangeToInclusive};
