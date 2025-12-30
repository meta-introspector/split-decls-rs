// Generated macro for impl_611 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_611 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_611"}
// Dependencies: {}
impl < K , V > NodeRef < marker :: Owned , K , V , marker :: LeafOrInternal > { # [doc = " Clears the root's link to its parent edge."] fn clear_parent_link (& mut self) { let mut root_node = self . borrow_mut () ; let leaf = root_node . as_leaf_mut () ; leaf . parent = None ; } }
};
}
