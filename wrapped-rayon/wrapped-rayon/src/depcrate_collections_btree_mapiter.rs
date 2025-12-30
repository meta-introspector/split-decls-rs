// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_btree_mapIter {
() => {
// Module: crate::collections::btree_map
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Parallel iterator over an immutable reference to a B-Tree map"] # [derive (Debug)] pub struct Iter < 'a , K , V > { inner : vec :: IntoIter < (& 'a K , & 'a V) > , }
};
}
