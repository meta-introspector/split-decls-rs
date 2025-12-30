// Generated macro for hash (function)
macro_rules! Depcrate_hash_quality_testhash {
() => {
// Module: crate::hash_quality_test
// Provides: {"hash"}
// Dependencies: {}
fn hash < H : Hash , T : Hasher > (b : & H , hash_builder : & dyn Fn () -> T) -> u64 { let mut hasher = hash_builder () ; b . hash (& mut hasher) ; hasher . finish () }
};
}
