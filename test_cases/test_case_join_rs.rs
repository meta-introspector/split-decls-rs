// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/future/join.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::pin::Pin;
use crate::task::{Context, Poll, ready};

/// Polls multiple futures simultaneously, returning a tuple
/// of all results once complete.
///
/// While `join!(a, b).await` is similar to `(a.await, b.await)`,
