// Generated macro for impl_610 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_610 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_610"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Sets the node's link to its parent edge,"] # [doc = " without invalidating other references to the node."] fn set_parent_link (& mut self , parent : NonNull < InternalNode < K , V > > , parent_idx : usize) { let leaf = Self :: as_leaf_ptr (self) ; unsafe { (* leaf) . parent = Some (parent) } ; unsafe { (* leaf) . parent_idx . write (parent_idx as u16) } ; } }
};
}
