// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/str/validations.rs
// Error: expected square brackets
// Problematic line: line 6

use super::Utf8Error;
use crate::intrinsics::const_eval_select;

/// Returns the initial codepoint accumulator for the first byte.
/// The first byte is special, only want bottom 5 bits for width 2, 4 bits
/// for width 3, and 3 bits for width 4.
#[inline]
