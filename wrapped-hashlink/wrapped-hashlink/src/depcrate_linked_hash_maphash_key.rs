// Generated macro for hash_key (function)
macro_rules! Depcrate_linked_hash_maphash_key {
() => {
// Module: crate::linked_hash_map
// Provides: {"hash_key"}
// Dependencies: {}
# [inline] fn hash_key < S , Q > (s : & S , k : & Q) -> u64 where S : BuildHasher , Q : Hash + ? Sized , { let mut hasher = s . build_hasher () ; k . hash (& mut hasher) ; hasher . finish () }
};
}
