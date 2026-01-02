// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/src/backtrace/win64.rs
// Error: expected square brackets
// Problematic line: line 12

use super::super::windows_sys::*;
use core::ffi::c_void;

#[derive(Clone, Copy)]
pub struct Frame {
    base_address: *mut c_void,
    ip: *mut c_void,
