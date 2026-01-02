// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/io/prelude.rs
// Error: expected square brackets
// Problematic line: line 13


#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
pub use super::{BufRead, Read, Seek, Write};
