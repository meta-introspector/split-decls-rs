// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/src/symbolize/gimli/stash.rs
// Error: expected square brackets
// Problematic line: line 10

use alloc::vec::Vec;
use core::cell::UnsafeCell;

/// A simple arena allocator for byte buffers.
pub struct Stash {
    buffers: UnsafeCell<Vec<Vec<u8>>>,
    mmaps: UnsafeCell<Vec<Mmap>>,
