// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sync/mpmc/array.rs
// Error: expected square brackets
// Problematic line: line 22

use crate::sync::atomic::{self, Atomic, AtomicUsize, Ordering};
use crate::time::Instant;

/// A slot in a channel.
struct Slot<T> {
    /// The current stamp.
    stamp: Atomic<usize>,
