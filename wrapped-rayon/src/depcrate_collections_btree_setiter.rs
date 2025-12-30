// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_btree_setIter {
() => {
// Module: crate::collections::btree_set
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Parallel iterator over an immutable reference to a B-Tree set"] # [derive (Debug)] pub struct Iter < 'a , T > { inner : vec :: IntoIter < & 'a T > , }
};
}
