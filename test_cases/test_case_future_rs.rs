// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/future/future.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::pin::Pin;
use crate::task::{Context, Poll};

/// A future represents an asynchronous computation, commonly obtained by use of
/// [`async`].
///
/// A future is a value that might not have finished computing yet. This kind of
