// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/default.rs
// Error: expected square brackets
// Problematic line: line 7


use crate::ascii::Char as AsciiChar;

/// A trait for giving a type a useful default value.
///
/// Sometimes, you want to fall back to some kind of default value, and
/// don't particularly care what it is. This comes up often with `struct`s
