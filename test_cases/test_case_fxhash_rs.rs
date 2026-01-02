// MINIMAL TEST CASE for parsing failure in: ../rust/library/proc_macro/src/bridge/fxhash.rs
// Error: expected square brackets
// Problematic line: line 11

use std::hash::{BuildHasherDefault, Hasher};
use std::ops::BitXor;

/// Type alias for a hashmap using the `fx` hash algorithm.
pub(super) type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

/// A speedy hash algorithm for use within rustc. The hashmap in alloc by
