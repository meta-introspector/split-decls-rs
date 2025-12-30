// Generated macro for hash_key (function)
macro_rules! Depcrate_nodes_hamthash_key {
() => {
// Module: crate::nodes::hamt
// Provides: {"hash_key"}
// Dependencies: {}
pub (crate) fn hash_key < K : Hash + ? Sized , S : BuildHasher > (bh : & S , key : & K) -> HashBits { let mut hasher = bh . build_hasher () ; key . hash (& mut hasher) ; hasher . finish () as HashBits }
};
}
