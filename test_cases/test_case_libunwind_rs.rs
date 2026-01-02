// MINIMAL TEST CASE for parsing failure in: ../rust/library/unwind/src/libunwind.rs
// Error: expected square brackets
// Problematic line: line 5


use core::ffi::{c_int, c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum _Unwind_Reason_Code {
    _URC_NO_REASON = 0,
