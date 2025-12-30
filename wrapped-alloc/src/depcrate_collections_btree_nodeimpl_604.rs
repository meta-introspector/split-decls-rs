// Generated macro for impl_604 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_604 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_604"}
// Dependencies: {}
impl < K , V , Type > NodeRef < marker :: Dying , K , V , Type > { # [doc = " Borrows exclusive access to the leaf portion of a dying leaf or internal node."] fn as_leaf_dying (& mut self) -> & mut LeafNode < K , V > { let ptr = Self :: as_leaf_ptr (self) ; unsafe { & mut * ptr } } }
};
}
