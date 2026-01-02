// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/sync/exclusive.rs
// Error: expected square brackets
// Problematic line: line 10

use core::pin::Pin;
use core::task::{Context, Poll};

/// `Exclusive` provides only _mutable_ access, also referred to as _exclusive_
/// access to the underlying value. It provides no _immutable_, or _shared_
/// access to the underlying value.
///
