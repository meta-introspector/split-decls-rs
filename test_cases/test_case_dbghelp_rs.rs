// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/src/symbolize/dbghelp.rs
// Error: expected square brackets
// Problematic line: line 30

use core::slice;

// FIXME: replace with ptr::from_ref once MSRV is high enough
#[inline(always)]
#[must_use]
const fn ptr_from_ref<T: ?Sized>(r: &T) -> *const T {
    r
