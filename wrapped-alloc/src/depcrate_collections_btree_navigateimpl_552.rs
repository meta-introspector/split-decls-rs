// Generated macro for impl_552 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_552 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_552"}
// Dependencies: {}
impl < K , V > NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > { # [doc = " Splits a unique reference into a pair of leaf edges delimiting the full range of the tree."] # [doc = " The results are non-unique references allowing massively destructive mutation, so must be"] # [doc = " used with the utmost care."] pub (super) fn full_range (self) -> LazyLeafRange < marker :: Dying , K , V > { let self2 = unsafe { ptr :: read (& self) } ; full_range (self , self2) } }
};
}
