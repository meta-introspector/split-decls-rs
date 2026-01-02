// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/unicode/unicode_data.rs
// Error: expected square brackets
// Problematic line: line 14

// to_upper        : 13432 bytes
// Total           : 31413 bytes

#[inline(always)]
const fn bitset_search<
    const N: usize,
    const CHUNK_SIZE: usize,
