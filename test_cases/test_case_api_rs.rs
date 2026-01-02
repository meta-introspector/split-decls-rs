// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/windows/api.rs
// Error: expected square brackets
// Problematic line: line 37


use super::c;

/// Creates a null-terminated UTF-16 string from a str.
pub macro wide_str($str:literal) {{
    const _: () = {
        if core::slice::memchr::memchr(0, $str.as_bytes()).is_some() {
