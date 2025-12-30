// Generated macro for impl_580 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_580 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_580"}
// Dependencies: {}
impl < K , V > InternalNode < K , V > { # [doc = " Creates a new boxed `InternalNode`."] # [doc = ""] # [doc = " # Safety"] # [doc = " An invariant of internal nodes is that they have at least one"] # [doc = " initialized and valid edge. This function does not set up"] # [doc = " such an edge."] unsafe fn new < A : Allocator + Clone > (alloc : A) -> Box < Self , A > { unsafe { let mut node = Box :: < Self , _ > :: new_uninit_in (alloc) ; LeafNode :: init (& raw mut (* node . as_mut_ptr ()) . data) ; node . assume_init () } } }
};
}
