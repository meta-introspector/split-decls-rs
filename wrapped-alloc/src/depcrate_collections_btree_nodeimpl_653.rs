// Generated macro for impl_653 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_653 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_653"}
// Dependencies: {}
impl < BorrowType , K , V > Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: KV > { pub (super) fn forget_node_type (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: KV > { unsafe { Handle :: new_kv (self . node . forget_type () , self . idx) } } }
};
}
