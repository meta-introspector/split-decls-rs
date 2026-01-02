// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/task/wake.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::panic::AssertUnwindSafe;
use crate::{fmt, ptr};

/// A `RawWaker` allows the implementor of a task executor to create a [`Waker`]
/// or a [`LocalWaker`] which provides customized wakeup behavior.
///
/// It consists of a data pointer and a [virtual function pointer table (vtable)][vtable]
