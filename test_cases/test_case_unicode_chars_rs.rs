// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_parse/src/lexer/unicode_chars.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::errors::TokenSubstitution;
use crate::token;

#[rustfmt::skip] // for line breaks
pub(super) static UNICODE_ARRAY: &[(char, &str, &str)] = &[
    (' ', "Line Separator", " "),
    (' ', "Paragraph Separator", " "),
