// Generated macro for impl_559 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_559 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_559"}
// Dependencies: {}
impl < BorrowType : marker :: BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { # [doc = " Returns the leftmost leaf edge in or underneath a node - in other words, the edge"] # [doc = " you need first when navigating forward (or last when navigating backward)."] # [inline] pub (super) fn first_leaf_edge (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { let mut node = self ; loop { match node . force () { Leaf (leaf) => return leaf . first_edge () , Internal (internal) => node = internal . first_edge () . descend () , } } } # [doc = " Returns the rightmost leaf edge in or underneath a node - in other words, the edge"] # [doc = " you need last when navigating forward (or first when navigating backward)."] # [inline] pub (super) fn last_leaf_edge (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { let mut node = self ; loop { match node . force () { Leaf (leaf) => return leaf . last_edge () , Internal (internal) => node = internal . last_edge () . descend () , } } } }
};
}
