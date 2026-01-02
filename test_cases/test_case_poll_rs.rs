// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/task/poll.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::convert;
use crate::ops::{self, ControlFlow};

/// Indicates whether a value is available or if the current task has been
/// scheduled to receive a wakeup instead.
///
/// This is returned by [`Future::poll`](core::future::Future::poll).
