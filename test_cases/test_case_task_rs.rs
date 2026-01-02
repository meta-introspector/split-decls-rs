// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/task.rs
// Error: expected square brackets
// Problematic line: line 11

//! `#[cfg(target_has_atomic = "ptr")]`.

use core::mem::ManuallyDrop;
#[cfg(target_has_atomic = "ptr")]
use core::task::Waker;
use core::task::{LocalWaker, RawWaker, RawWakerVTable};

