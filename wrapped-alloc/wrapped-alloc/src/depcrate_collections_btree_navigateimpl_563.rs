// Generated macro for impl_563 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_563 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_563"}
// Dependencies: {}
impl < BorrowType : marker :: BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { # [doc = " Returns the leaf edge corresponding to the first point at which the"] # [doc = " given bound is true."] pub (super) fn lower_bound < Q : ? Sized > (self , mut bound : SearchBound < & Q > ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > where Q : Ord , K : Borrow < Q > , { let mut node = self ; loop { let (edge , new_bound) = node . find_lower_bound_edge (bound) ; match edge . force () { Leaf (edge) => return edge , Internal (edge) => { node = edge . descend () ; bound = new_bound ; } } } } # [doc = " Returns the leaf edge corresponding to the last point at which the"] # [doc = " given bound is true."] pub (super) fn upper_bound < Q : ? Sized > (self , mut bound : SearchBound < & Q > ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > where Q : Ord , K : Borrow < Q > , { let mut node = self ; loop { let (edge , new_bound) = node . find_upper_bound_edge (bound) ; match edge . force () { Leaf (edge) => return edge , Internal (edge) => { node = edge . descend () ; bound = new_bound ; } } } } }
};
}
