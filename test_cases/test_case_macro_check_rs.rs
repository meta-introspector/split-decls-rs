// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_expand/src/mbe/macro_check.rs
// Error: expected square brackets
// Problematic line: line 121

use crate::errors;
use crate::mbe::{KleeneToken, TokenTree};

/// Stack represented as linked list.
///
/// Those are used for environments because they grow incrementally and are not mutable.
enum Stack<'a, T> {
