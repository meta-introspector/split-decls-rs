// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/sync/mutex/windows7.rs
// Error: expected square brackets
// Problematic line: line 20

use crate::cell::UnsafeCell;
use crate::sys::c;

pub struct Mutex {
    srwlock: UnsafeCell<c::SRWLOCK>,
}

