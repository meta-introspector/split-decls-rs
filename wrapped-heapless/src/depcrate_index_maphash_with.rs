// Generated macro for hash_with (function)
macro_rules! Depcrate_index_maphash_with {
() => {
// Module: crate::index_map
// Provides: {"hash_with"}
// Dependencies: {}
fn hash_with < K , S > (key : & K , build_hasher : & S) -> HashValue where K : ? Sized + Hash , S : BuildHasher , { HashValue (build_hasher . hash_one (key) as u16) }
};
}
