// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/char/decode.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::fmt;
use crate::iter::FusedIterator;

/// An iterator that decodes UTF-16 encoded code points from an iterator of `u16`s.
///
/// This `struct` is created by the [`decode_utf16`] method on [`char`]. See its
/// documentation for more.
