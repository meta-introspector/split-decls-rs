// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_expand/src/mbe/macro_parser.rs
// Error: expected square brackets
// Problematic line: line 90

use crate::mbe::macro_rules::Tracker;
use crate::mbe::{KleeneOp, TokenTree};

/// A unit within a matcher that a `MatcherPos` can refer to. Similar to (and derived from)
/// `mbe::TokenTree`, but designed specifically for fast and easy traversal during matching.
/// Notable differences to `mbe::TokenTree`:
/// - It is non-recursive, i.e. there is no nesting.
