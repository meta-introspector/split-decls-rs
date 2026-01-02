// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/str/converts.rs
// Error: expected square brackets
// Problematic line: line 7

use super::validations::run_utf8_validation;
use crate::{mem, ptr};

/// Converts a slice of bytes to a string slice.
///
/// This is an alias to [`str::from_utf8`].
///
