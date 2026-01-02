// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_errors/src/lock.rs
// Error: expected square brackets
// Problematic line: line 14


use std::any::Any;

#[cfg(windows)]
pub(crate) fn acquire_global_lock(name: &str) -> Box<dyn Any> {
    use std::ffi::CString;
    use std::io;
