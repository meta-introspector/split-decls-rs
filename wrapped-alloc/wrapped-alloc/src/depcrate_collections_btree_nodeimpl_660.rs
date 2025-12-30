// Generated macro for impl_660 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_660 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_660"}
// Dependencies: {}
impl < 'a , K , V > SplitResult < 'a , K , V , marker :: Internal > { pub (super) fn forget_node_type (self) -> SplitResult < 'a , K , V , marker :: LeafOrInternal > { SplitResult { left : self . left . forget_type () , kv : self . kv , right : self . right . forget_type () } } }
};
}
