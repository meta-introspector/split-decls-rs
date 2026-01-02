// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/ffi/os_str.rs
// Error: expected square brackets
// Problematic line: line 3

//! The [`OsStr`] and [`OsString`] types and associated utilities.

#[cfg(test)]
mod tests;

use core::clone::CloneToUninit;
