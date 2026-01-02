// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/result.rs
// Error: expected square brackets
// Problematic line: line 541

use crate::ops::{self, ControlFlow, Deref, DerefMut};
use crate::{convert, fmt, hint};

/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
///
/// See the [module documentation](self) for details.
#[doc(search_unbox)]
