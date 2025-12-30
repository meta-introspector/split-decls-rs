// Generated macro for impl_551 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_551 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_551"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > NodeRef < marker :: ValMut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Splits a unique reference into a pair of leaf edges delimiting a specified range."] # [doc = " The result are non-unique references allowing (some) mutation, which must be used"] # [doc = " carefully."] # [doc = ""] # [doc = " The result is meaningful only if the tree is ordered by key, like the tree"] # [doc = " in a `BTreeMap` is."] # [doc = ""] # [doc = " # Safety"] # [doc = " Do not use the duplicate handles to visit the same KV twice."] pub (super) fn range_search < Q , R > (self , range : R) -> LeafRange < marker :: ValMut < 'a > , K , V > where Q : ? Sized + Ord , K : Borrow < Q > , R : RangeBounds < Q > , { unsafe { self . find_leaf_edges_spanning_range (range) } } # [doc = " Splits a unique reference into a pair of leaf edges delimiting the full range of the tree."] # [doc = " The results are non-unique references allowing mutation (of values only), so must be used"] # [doc = " with care."] pub (super) fn full_range (self) -> LazyLeafRange < marker :: ValMut < 'a > , K , V > { let self2 = unsafe { ptr :: read (& self) } ; full_range (self , self2) } }
};
}
