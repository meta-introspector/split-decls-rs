// Generated macro for hash_header (function)
macro_rules! Depcrate_hpack_tablehash_header {
() => {
// Module: crate::hpack::table
// Provides: {"hash_header"}
// Dependencies: {}
fn hash_header (header : & Header) -> HashValue { const MASK : u64 = (MAX_SIZE as u64) - 1 ; let mut h = FnvHasher :: default () ; header . name () . hash (& mut h) ; HashValue ((h . finish () & MASK) as usize) }
};
}
