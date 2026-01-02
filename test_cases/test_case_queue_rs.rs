// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/sync/rwlock/queue.rs
// Error: expected square brackets
// Problematic line: line 123

use crate::sync::atomic::{Atomic, AtomicBool, AtomicPtr};
use crate::thread::{self, Thread};

/// The atomic lock state.
type AtomicState = Atomic<State>;
/// The inner lock state.
type State = *mut ();
