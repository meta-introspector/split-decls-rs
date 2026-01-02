// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_errors/src/markdown/parse.rs
// Error: expected `,`
// Error type: expected_comma
// Sample #2 of 3
// Problematic line: line 5


use crate::markdown::{MdStream, MdTree};

/// Short aliases that we can use in match patterns. If an end pattern is not
/// included, this type may be variable
const ANC_E: &[u8] = b">";
const ANC_S: &[u8] = b"<";
