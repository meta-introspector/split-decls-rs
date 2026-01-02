// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sync/nonpoison.rs
// Error: expected square brackets
// Problematic line: line 10


use crate::fmt;

/// A type alias for the result of a nonblocking locking method.
#[unstable(feature = "sync_nonpoison", issue = "134645")]
pub type TryLockResult<Guard> = Result<Guard, WouldBlock>;

