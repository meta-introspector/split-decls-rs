// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/tagged_ptr.rs
// Error: expected square brackets
// Problematic line: line 17

use crate::aligned::Aligned;
use crate::stable_hasher::{HashStable, StableHasher};

/// This describes tags that the [`TaggedRef`] struct can hold.
///
/// # Safety
///
