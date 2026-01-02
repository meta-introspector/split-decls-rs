// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/process/unix/unsupported/wait_status.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::fmt;
use crate::num::NonZero;

/// Emulated wait status for use by `unsupported.rs`
///
/// Uses the "traditional unix" encoding.  For use on platfors which are `#[cfg(unix)]`
/// but do not actually support subprocesses at all.
