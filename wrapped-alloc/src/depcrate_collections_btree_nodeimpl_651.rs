// Generated macro for impl_651 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_651 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_651"}
// Dependencies: {}
impl < BorrowType , K , V > Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > { pub (super) fn forget_node_type (self ,) -> Handle < NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , marker :: Edge > { unsafe { Handle :: new_edge (self . node . forget_type () , self . idx) } } }
};
}
