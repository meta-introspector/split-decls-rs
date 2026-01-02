// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sync/mpmc/waker.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::sync::Mutex;
use crate::sync::atomic::{Atomic, AtomicBool, Ordering};

/// Represents a thread blocked on a specific channel operation.
pub(crate) struct Entry {
    /// The operation.
    pub(crate) oper: Operation,
