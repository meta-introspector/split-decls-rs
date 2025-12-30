// Generated macro for impl_634 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_634 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_634"}
// Dependencies: {}
impl < 'a , K , V > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > , marker :: Edge > { # [doc = " Fixes the parent pointer and index in the child node that this edge"] # [doc = " links to. This is useful when the ordering of edges has been changed,"] fn correct_parent_link (self) { let ptr = unsafe { NonNull :: new_unchecked (NodeRef :: as_internal_ptr (& self . node)) } ; let idx = self . idx ; let mut child = self . descend () ; child . set_parent_link (ptr , idx) ; } }
};
}
