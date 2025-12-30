// Generated macro for impl_652 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_652 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_652"}
// Dependencies: {}
impl < BorrowType , K , V > Handle < NodeRef < BorrowType , K , V , marker :: Internal > , marker :: Edge > { pub (super) fn forget_node_type (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: Edge > { unsafe { Handle :: new_edge (self . node . forget_type () , self . idx) } } }
};
}
