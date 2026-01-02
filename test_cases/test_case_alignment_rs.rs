// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ptr/alignment.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::ub_checks::assert_unsafe_precondition;
use crate::{cmp, fmt, hash, mem, num};

/// A type storing a `usize` which is a power of two, and thus
/// represents a possible alignment in the Rust abstract machine.
///
/// Note that particularly large alignments, while representable in this type,
