// Generated macro for impl_616 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_616 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_616"}
// Dependencies: {}
impl < BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: Leaf > { # [doc = " Removes any static information asserting that this node is a `Leaf` node."] pub (super) fn forget_type (self) -> NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { NodeRef { height : self . height , node : self . node , _marker : PhantomData } } }
};
}
