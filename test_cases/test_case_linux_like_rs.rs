// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/thread_local/destructors/linux_like.rs
// Error: expected square brackets
// Problematic line: line 15


use crate::mem::transmute;

pub unsafe fn register(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
    /// This is necessary because the __cxa_thread_atexit_impl implementation
    /// std links to by default may be a C or C++ implementation that was not
    /// compiled using the Clang integer normalization option.
