// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/thread_local/no_threads.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::cell::{Cell, UnsafeCell};
use crate::ptr;

#[doc(hidden)]
#[allow_internal_unstable(thread_local_internals)]
#[allow_internal_unsafe]
#[unstable(feature = "thread_local_internals", issue = "none")]
