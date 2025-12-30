// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_hash_mapIter {
() => {
// Module: crate::collections::hash_map
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Parallel iterator over an immutable reference to a hash map"] # [derive (Debug)] pub struct Iter < 'a , K , V > { inner : vec :: IntoIter < (& 'a K , & 'a V) > , }
};
}
