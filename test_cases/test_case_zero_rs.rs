// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sync/mpmc/zero.rs
// Error: expected square brackets
// Problematic line: line 17

use crate::time::Instant;
use crate::{fmt, ptr};

/// A pointer to a packet.
pub(crate) struct ZeroToken(*mut ());

impl Default for ZeroToken {
