// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/thread_local/guard/solid.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::sys::pal::itron::task;
use crate::sys::thread_local::destructors;

pub fn enable() {
    #[thread_local]
    static REGISTERED: Cell<bool> = Cell::new(false);

