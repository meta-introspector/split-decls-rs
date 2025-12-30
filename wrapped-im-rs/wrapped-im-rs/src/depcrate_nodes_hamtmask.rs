// Generated macro for mask (function)
macro_rules! Depcrate_nodes_hamtmask {
() => {
// Module: crate::nodes::hamt
// Provides: {"mask"}
// Dependencies: {}
# [inline] fn mask (hash : HashBits , shift : usize) -> HashBits { hash >> shift & HASH_MASK }
};
}
