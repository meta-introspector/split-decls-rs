// Generated macro for impl_592 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_592 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_592"}
// Dependencies: {}
impl < K , V > NodeRef < marker :: Owned , K , V , marker :: Leaf > { pub (super) fn new_leaf < A : Allocator + Clone > (alloc : A) -> Self { Self :: from_new_leaf (LeafNode :: new (alloc)) } fn from_new_leaf < A : Allocator + Clone > (leaf : Box < LeafNode < K , V > , A >) -> Self { NodeRef { height : 0 , node : NonNull :: from (Box :: leak (leaf)) , _marker : PhantomData } } }
};
}
