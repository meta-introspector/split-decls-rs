// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/windows/c.rs
// Error: expected square brackets
// Problematic line: line 22

pub const EXIT_SUCCESS: u32 = 0;
pub const EXIT_FAILURE: u32 = 1;

#[cfg(target_vendor = "win7")]
pub const CONDITION_VARIABLE_INIT: CONDITION_VARIABLE = CONDITION_VARIABLE { Ptr: ptr::null_mut() };
#[cfg(target_vendor = "win7")]
pub const SRWLOCK_INIT: SRWLOCK = SRWLOCK { Ptr: ptr::null_mut() };
