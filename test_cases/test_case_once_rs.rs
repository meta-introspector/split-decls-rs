// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sync/poison/once.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::panic::{RefUnwindSafe, UnwindSafe};
use crate::sys::sync as sys;

/// A low-level synchronization primitive for one-time global execution.
///
/// Previously this was the only "execute once" synchronization in `std`.
/// Other libraries implemented novel synchronizing types with `Once`, like
