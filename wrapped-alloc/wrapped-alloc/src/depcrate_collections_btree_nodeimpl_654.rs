// Generated macro for impl_654 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_654 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_654"}
// Dependencies: {}
impl < BorrowType , K , V , Type > Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , Type > { # [doc = " Checks whether the underlying node is an `Internal` node or a `Leaf` node."] pub (super) fn force (self ,) -> ForceResult < Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , Type > , Handle < NodeRef < BorrowType , K , V , marker :: Internal > , Type > , > { match self . node . force () { ForceResult :: Leaf (node) => { ForceResult :: Leaf (Handle { node , idx : self . idx , _marker : PhantomData }) } ForceResult :: Internal (node) => { ForceResult :: Internal (Handle { node , idx : self . idx , _marker : PhantomData }) } } } }
};
}
