// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/array/iter/iter_inner.rs
// Error: expected square brackets
// Problematic line: line 8

use crate::ops::{IndexRange, NeverShortCircuit, Try};
use crate::{fmt, iter};

#[allow(private_bounds)]
trait PartialDrop {
    /// # Safety
    /// `self[alive]` are all initialized before the call,
