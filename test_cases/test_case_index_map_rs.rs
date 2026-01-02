// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/sorted_map/index_map.rs
// Error: expected square brackets
// Problematic line: line 9


use crate::stable_hasher::{HashStable, StableHasher};

/// An indexed multi-map that preserves insertion order while permitting both *O*(log *n*) lookup of
/// an item by key and *O*(1) lookup by index.
///
/// This data structure is a hybrid of an [`IndexVec`] and a [`SortedMap`]. Like `IndexVec`,
