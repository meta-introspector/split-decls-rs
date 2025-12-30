// Generated macro for impl_399 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_399 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_399"}
// Dependencies: {}
impl < K , V , A : Allocator + Clone > IntoIter < K , V , A > { # [doc = " Returns an iterator of references over the remaining items."] # [inline] pub (super) fn iter (& self) -> Iter < '_ , K , V > { Iter { range : self . range . reborrow () , length : self . length } } }
};
}
