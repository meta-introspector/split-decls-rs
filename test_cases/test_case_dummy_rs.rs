// MINIMAL TEST CASE for parsing failure in: ../rust/library/panic_unwind/src/dummy.rs
// Error: expected square brackets
// Problematic line: line 9

use core::any::Any;
use core::intrinsics;

pub(crate) unsafe fn cleanup(_ptr: *mut u8) -> Box<dyn Any + Send> {
    intrinsics::abort()
}

