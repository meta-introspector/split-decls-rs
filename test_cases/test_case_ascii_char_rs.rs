// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ascii/ascii_char.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::mem::transmute;
use crate::{assert_unsafe_precondition, fmt};

/// One of the 128 Unicode characters from U+0000 through U+007F,
/// often known as the [ASCII] subset.
///
/// Officially, this is the first [block] in Unicode, _Basic Latin_.
