// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/unix/kernel_copy.rs
// Error: expected square brackets
// Problematic line: line 45

//!   progress, they can hit a performance cliff.
//! * complexity

#[cfg(not(any(all(target_os = "linux", target_env = "gnu"), target_os = "hurd")))]
use libc::sendfile as sendfile64;
#[cfg(any(all(target_os = "linux", target_env = "gnu"), target_os = "hurd"))]
use libc::sendfile64;
