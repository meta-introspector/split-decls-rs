// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sync/mpsc.rs
// Error: expected square brackets
// Problematic line: line 149

use crate::time::{Duration, Instant};
use crate::{error, fmt};

/// The receiving half of Rust's [`channel`] (or [`sync_channel`]) type.
/// This half can only be owned by one thread.
///
/// Messages sent to the channel can be retrieved using [`recv`].
