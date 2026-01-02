// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/hash/sip.rs
// Error: expected square brackets
// Problematic line: line 8

use crate::marker::PhantomData;
use crate::{cmp, ptr};

/// An implementation of SipHash 1-3.
///
/// This is currently the default hashing function used by standard library
/// (e.g., `collections::HashMap` uses it by default).
