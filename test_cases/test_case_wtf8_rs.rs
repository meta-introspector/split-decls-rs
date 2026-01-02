// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/wtf8.rs
// Error: expected square brackets
// Problematic line: line 13

//! of WTF-8 strings,
//! nor can it decode WTF-8 from arbitrary bytes.
//! WTF-8 strings can be obtained from UTF-8, UTF-16, or code points.
#![unstable(
    feature = "wtf8_internals",
    issue = "none",
    reason = "this is internal code for representing OsStr on some platforms and not a public API"
