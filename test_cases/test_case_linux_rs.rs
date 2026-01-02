// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/random/linux.rs
// Error: expected square brackets
// Problematic line: line 72

use crate::sys::pal::os::errno;
use crate::sys::pal::weak::syscall;

fn getrandom(mut bytes: &mut [u8], insecure: bool) {
    // A weak symbol allows interposition, e.g. for perf measurements that want to
    // disable randomness for consistency. Otherwise, we'll try a raw syscall.
    // (`getrandom` was added in glibc 2.25, musl 1.1.20, android API level 28)
