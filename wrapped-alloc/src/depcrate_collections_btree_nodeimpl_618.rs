// Generated macro for impl_618 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_618 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_618"}
// Dependencies: {}
impl < BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { # [doc = " Checks whether a node is an `Internal` node or a `Leaf` node."] pub (super) fn force (self ,) -> ForceResult < NodeRef < BorrowType , K , V , marker :: Leaf > , NodeRef < BorrowType , K , V , marker :: Internal > , > { if self . height == 0 { ForceResult :: Leaf (NodeRef { height : self . height , node : self . node , _marker : PhantomData , }) } else { ForceResult :: Internal (NodeRef { height : self . height , node : self . node , _marker : PhantomData , }) } } }
};
}
