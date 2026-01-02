// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/sync/thread_parking/darwin.rs
// Error: expected square brackets
// Problematic line: line 27

const DISPATCH_TIME_FOREVER: dispatch_time_t = !0;

// Contained in libSystem.dylib, which is linked by default.
unsafe extern "C" {
    fn dispatch_time(when: dispatch_time_t, delta: i64) -> dispatch_time_t;
    fn dispatch_semaphore_create(val: isize) -> dispatch_semaphore_t;
    fn dispatch_semaphore_wait(dsema: dispatch_semaphore_t, timeout: dispatch_time_t) -> isize;
