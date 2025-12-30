// Generated macro for impl_659 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_659 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_659"}
// Dependencies: {}
impl < 'a , K , V > SplitResult < 'a , K , V , marker :: Leaf > { pub (super) fn forget_node_type (self) -> SplitResult < 'a , K , V , marker :: LeafOrInternal > { SplitResult { left : self . left . forget_type () , kv : self . kv , right : self . right . forget_type () } } }
};
}
