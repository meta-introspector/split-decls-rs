// Generated macro for hash_node (function)
macro_rules! Depcrate_linked_hash_maphash_node {
() => {
// Module: crate::linked_hash_map
// Provides: {"hash_node"}
// Dependencies: {}
# [inline] unsafe fn hash_node < S , K , V > (s : & S , node : NonNull < Node < K , V > >) -> u64 where S : BuildHasher , K : Hash , { hash_key (s , node . as_ref () . key_ref ()) }
};
}
