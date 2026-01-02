// MINIMAL TEST CASE for parsing failure in: ../rust/library/unwind/src/unwinding.rs
// Error: expected square brackets
// Problematic line: line 13

pub const _UA_FORCE_UNWIND: c_int = 8;
pub const _UA_END_OF_STACK: c_int = 16;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum _Unwind_Reason_Code {
    _URC_NO_REASON = 0,
