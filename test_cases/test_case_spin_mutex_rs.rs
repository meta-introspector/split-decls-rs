// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/sgx/waitqueue/spin_mutex.rs
// Error: expected square brackets
// Problematic line: line 4

//! Trivial spinlock-based implementation of `sync::Mutex`.
// FIXME: Perhaps use Intel TSX to avoid locking?

#[cfg(test)]
mod tests;

use crate::cell::UnsafeCell;
