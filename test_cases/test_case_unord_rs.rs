// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/unord.rs
// Error: expected square brackets
// Problematic line: line 17

use crate::fingerprint::Fingerprint;
use crate::stable_hasher::{HashStable, StableCompare, StableHasher, ToStableHashKey};

/// `UnordItems` is the order-less version of `Iterator`. It only contains methods
/// that don't (easily) expose an ordering of the underlying items.
///
/// Most methods take an `Fn` where the `Iterator`-version takes an `FnMut`. This
