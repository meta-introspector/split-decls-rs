// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/prelude/v1.rs
// Error: expected square brackets
// Problematic line: line 11

#![cfg_attr(rustfmt, rustfmt::skip)]

// Re-exported core operators
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::marker::{Send, Sized, Sync, Unpin};
#[stable(feature = "rust1", since = "1.0.0")]
