// Generated macro for hash_with (function)
macro_rules! Depcrate_hash_quality_testhash_with {
() => {
// Module: crate::hash_quality_test
// Provides: {"hash_with"}
// Dependencies: {}
fn hash_with < H : Hash , T : Hasher > (b : & H , mut hasher : T) -> u64 { b . hash (& mut hasher) ; hasher . finish () }
};
}
