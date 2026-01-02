// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/sync/once_box.rs
// Error: expected square brackets
// Problematic line: line 14

use crate::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use crate::sync::atomic::{Atomic, AtomicPtr};

pub(crate) struct OnceBox<T> {
    ptr: Atomic<*mut T>,
}

