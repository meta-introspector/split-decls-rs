// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/sync/thread_parking/id.rs
// Error: expected square brackets
// Problematic line: line 17

use crate::sys::thread_parking::{ThreadId, current, park, park_timeout, unpark};
use crate::time::Duration;

pub struct Parker {
    state: Atomic<i8>,
    tid: UnsafeCell<Option<ThreadId>>,
}
