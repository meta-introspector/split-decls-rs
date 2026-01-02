// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/thread/local.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::error::Error;
use crate::fmt;

/// A thread local storage (TLS) key which owns its contents.
///
/// This key uses the fastest implementation available on the target platform.
/// It is instantiated with the [`thread_local!`] macro and the
