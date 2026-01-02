// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/time.rs
// Error: expected square brackets
// Problematic line: line 34


#![stable(feature = "time", since = "1.3.0")]

#[stable(feature = "time", since = "1.3.0")]
pub use core::time::Duration;
#[stable(feature = "duration_checked_float", since = "1.66.0")]
pub use core::time::TryFromFloatSecsError;
