// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs
// Error: expected square brackets
// Problematic line: line 13


use crate::errors::{MoreThanOneCharNote, MoreThanOneCharSugg, NoBraceUnicodeSub, UnescapeError};

pub(crate) fn emit_unescape_error(
    dcx: DiagCtxtHandle<'_>,
    // interior part of the literal, between quotes
    lit: &str,
