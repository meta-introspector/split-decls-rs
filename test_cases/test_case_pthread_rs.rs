// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/sync/condvar/pthread.rs
// Error: expected square brackets
// Problematic line: line 11

use crate::sys::sync::{Mutex, OnceBox};
use crate::time::{Duration, Instant};

pub struct Condvar {
    cvar: OnceBox<pal::Condvar>,
    mutex: Atomic<usize>,
}
