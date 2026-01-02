// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/fmt/rt.rs
// Error: expected square brackets
// Problematic line: line 13

use crate::hint::unreachable_unchecked;
use crate::ptr::NonNull;

#[lang = "format_placeholder"]
#[derive(Copy, Clone)]
pub struct Placeholder {
    pub position: usize,
