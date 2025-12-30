// Generated macro for impl_594 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_594 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_594"}
// Dependencies: {}
impl < BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: Internal > { # [doc = " Unpack a node reference that was packed as `NodeRef::parent`."] fn from_internal (node : NonNull < InternalNode < K , V > > , height : usize) -> Self { debug_assert ! (height > 0) ; NodeRef { height , node : node . cast () , _marker : PhantomData } } }
};
}
