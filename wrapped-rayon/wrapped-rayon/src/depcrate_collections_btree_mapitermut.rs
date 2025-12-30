// Generated macro for IterMut (struct)
macro_rules! Depcrate_collections_btree_mapIterMut {
() => {
// Module: crate::collections::btree_map
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Parallel iterator over a mutable reference to a B-Tree map"] # [derive (Debug)] pub struct IterMut < 'a , K , V > { inner : vec :: IntoIter < (& 'a K , & 'a mut V) > , }
};
}
