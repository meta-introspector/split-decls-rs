// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_expand/src/mbe.rs
// Error: expected square brackets
// Problematic line: line 21

use rustc_macros::{Decodable, Encodable};
use rustc_span::{Ident, Span};

/// Contains the sub-token-trees of a "delimited" token tree such as `(a b c)`.
/// The delimiters are not represented explicitly in the `tts` vector.
#[derive(PartialEq, Encodable, Decodable, Debug)]
struct Delimited {
