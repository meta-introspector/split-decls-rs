// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/thread_local/guard/apple.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::ptr;
use crate::sys::thread_local::destructors;

pub fn enable() {
    #[thread_local]
    static REGISTERED: Cell<bool> = Cell::new(false);

