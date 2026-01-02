// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys_common/wstr.rs
// Error: expected square brackets
// Problematic line: line 8

use crate::num::NonZero;
use crate::ptr::NonNull;

/// A safe iterator over a LPWSTR
/// (aka a pointer to a series of UTF-16 code units terminated by a NULL).
pub struct WStrUnits<'a> {
    // The pointer must never be null...
