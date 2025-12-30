// Generated macro for impl_601 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_601 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_601"}
// Dependencies: {}
impl < K , V > NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > { # [doc = " Similar to `ascend`, gets a reference to a node's parent node, but also"] # [doc = " deallocates the current node in the process. This is unsafe because the"] # [doc = " current node will still be accessible despite being deallocated."] pub (super) unsafe fn deallocate_and_ascend < A : Allocator + Clone > (self , alloc : A ,) -> Option < Handle < NodeRef < marker :: Dying , K , V , marker :: Internal > , marker :: Edge > > { let height = self . height ; let node = self . node ; let ret = self . ascend () . ok () ; unsafe { alloc . deallocate (node . cast () , if height > 0 { Layout :: new :: < InternalNode < K , V > > () } else { Layout :: new :: < LeafNode < K , V > > () } ,) ; } ret } }
};
}
