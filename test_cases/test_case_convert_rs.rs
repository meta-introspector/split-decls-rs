// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/char/convert.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::str::FromStr;
use crate::ub_checks::assert_unsafe_precondition;

/// Converts a `u32` to a `char`. See [`char::from_u32`].
#[must_use]
#[inline]
pub(super) const fn from_u32(i: u32) -> Option<char> {
