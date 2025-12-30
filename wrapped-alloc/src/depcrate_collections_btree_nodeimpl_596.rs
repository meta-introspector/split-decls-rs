// Generated macro for impl_596 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_596 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_596"}
// Dependencies: {}
impl < 'a , K , V > NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > { # [doc = " Borrows exclusive access to the data of an internal node."] fn as_internal_mut (& mut self) -> & mut InternalNode < K , V > { let ptr = Self :: as_internal_ptr (self) ; unsafe { & mut * ptr } } }
};
}
