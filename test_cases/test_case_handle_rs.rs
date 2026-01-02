// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/windows/handle.rs
// Error: expected square brackets
// Problematic line: line 3

#![unstable(issue = "none", feature = "windows_handle")]

#[cfg(test)]
mod tests;

use core::ffi::c_void;
