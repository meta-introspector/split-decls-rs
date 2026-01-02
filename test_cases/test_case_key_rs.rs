// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/thread_local/guard/key.rs
// Error: expected square brackets
// Problematic line: line 8

use crate::ptr;
use crate::sys::thread_local::key::{LazyKey, set};

#[cfg(target_thread_local)]
pub fn enable() {
    use crate::sys::thread_local::destructors;

