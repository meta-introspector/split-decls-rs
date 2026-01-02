// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sync/poison.rs
// Error: expected square brackets
// Problematic line: line 63

//!   while it is locked exclusively (write mode). If a panic occurs in any reader,
//!   then the lock will not be poisoned.

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::condvar::Condvar;
#[unstable(feature = "mapped_lock_guards", issue = "117108")]
pub use self::mutex::MappedMutexGuard;
