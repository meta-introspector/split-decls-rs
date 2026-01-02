// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/vec_cache.rs
// Error: expected square brackets
// Problematic line: line 15


use rustc_index::Idx;

struct Slot<V> {
    // We never construct &Slot<V> so it's fine for this to not be in an UnsafeCell.
    value: V,
    // This is both an index and a once-lock.
