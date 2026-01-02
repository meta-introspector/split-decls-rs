// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/char/methods.rs
// Error: expected square brackets
// Problematic line: line 11

use crate::unicode::printable::is_printable;
use crate::unicode::{self, conversions};

impl char {
    /// The lowest valid code point a `char` can have, `'\0'`.
    ///
    /// Unlike integer types, `char` actually has a gap in the middle,
