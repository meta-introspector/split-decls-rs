// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/escape.rs
// Error: expected square brackets
// Problematic line: line 11


const HEX_DIGITS: [ascii::Char; 16] = *b"0123456789abcdef".as_ascii().unwrap();

/// Escapes a character with `\x` representation.
///
/// Returns a buffer with the escaped representation and its corresponding range.
#[inline]
