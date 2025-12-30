// Generated macro for impl_562 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_562 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_562"}
// Dependencies: {}
impl < BorrowType : marker :: BorrowType , K , V > Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: KV > { # [doc = " Returns the leaf edge closest to a KV for forward navigation."] pub (super) fn next_leaf_edge (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { match self . force () { Leaf (leaf_kv) => leaf_kv . right_edge () , Internal (internal_kv) => { let next_internal_edge = internal_kv . right_edge () ; next_internal_edge . descend () . first_leaf_edge () } } } # [doc = " Returns the leaf edge closest to a KV for backward navigation."] pub (super) fn next_back_leaf_edge (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { match self . force () { Leaf (leaf_kv) => leaf_kv . left_edge () , Internal (internal_kv) => { let next_internal_edge = internal_kv . left_edge () ; next_internal_edge . descend () . last_leaf_edge () } } } }
};
}
