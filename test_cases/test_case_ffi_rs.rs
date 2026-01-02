// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/xous/ffi.rs
// Error: expected square brackets
// Problematic line: line 5

#![allow(unused_variables)]
#![stable(feature = "rust1", since = "1.0.0")]

#[path = "../unix/ffi/os_str.rs"]
mod os_str;

#[stable(feature = "rust1", since = "1.0.0")]
