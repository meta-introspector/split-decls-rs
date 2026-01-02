// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/unix/thread_parking.rs
// Error: expected square brackets
// Problematic line: line 11

use crate::ptr;
use crate::time::Duration;

unsafe extern "C" {
    fn ___lwp_park60(
        clock_id: clockid_t,
        flags: c_int,
