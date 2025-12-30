// Generated macro for impl_578 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_578 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_578"}
// Dependencies: {}
impl < K , V > LeafNode < K , V > { # [doc = " Initializes a new `LeafNode` in-place."] unsafe fn init (this : * mut Self) { unsafe { (& raw mut (* this) . parent) . write (None) ; (& raw mut (* this) . len) . write (0) ; } } # [doc = " Creates a new boxed `LeafNode`."] fn new < A : Allocator + Clone > (alloc : A) -> Box < Self , A > { unsafe { let mut leaf = Box :: new_uninit_in (alloc) ; LeafNode :: init (leaf . as_mut_ptr ()) ; leaf . assume_init () } } }
};
}
