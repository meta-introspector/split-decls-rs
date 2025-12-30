// Generated macro for impl_430 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_430 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_430"}
// Dependencies: {}
impl < 'a , K , V > IterMut < 'a , K , V > { # [doc = " Returns an iterator of references over the remaining items."] # [inline] pub (super) fn iter (& self) -> Iter < '_ , K , V > { Iter { range : self . range . reborrow () , length : self . length } } }
};
}
