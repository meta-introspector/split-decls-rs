// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/src/backtrace/win32.rs
// Error: expected square brackets
// Problematic line: line 17

use core::ffi::c_void;
use core::mem;

#[derive(Clone, Copy)]
pub enum StackFrame {
    New(STACKFRAME_EX),
    Old(STACKFRAME64),
