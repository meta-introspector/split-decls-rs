// Generated macro for impl_617 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_617 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_617"}
// Dependencies: {}
impl < BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: Internal > { # [doc = " Removes any static information asserting that this node is an `Internal` node."] pub (super) fn forget_type (self) -> NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { NodeRef { height : self . height , node : self . node , _marker : PhantomData } } }
};
}
