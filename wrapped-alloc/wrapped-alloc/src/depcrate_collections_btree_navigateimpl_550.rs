// Generated macro for impl_550 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_550 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_550"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Immut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Finds the pair of leaf edges delimiting a specific range in a tree."] # [doc = ""] # [doc = " The result is meaningful only if the tree is ordered by key, like the tree"] # [doc = " in a `BTreeMap` is."] pub (super) fn range_search < Q , R > (self , range : R) -> LeafRange < marker :: Immut < 'a > , K , V > where Q : ? Sized + Ord , K : Borrow < Q > , R : RangeBounds < Q > , { unsafe { self . find_leaf_edges_spanning_range (range) } } # [doc = " Finds the pair of leaf edges delimiting an entire tree."] pub (super) fn full_range (self) -> LazyLeafRange < marker :: Immut < 'a > , K , V > { full_range (self , self) } }
};
}
