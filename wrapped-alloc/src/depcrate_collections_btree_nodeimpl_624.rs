// Generated macro for impl_624 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_624 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_624"}
// Dependencies: {}
impl < BorrowType , K , V , NodeType > Handle < NodeRef < BorrowType , K , V , NodeType > , marker :: KV > { # [doc = " Creates a new handle to a key-value pair in `node`."] # [doc = " Unsafe because the caller must ensure that `idx < node.len()`."] pub (super) unsafe fn new_kv (node : NodeRef < BorrowType , K , V , NodeType > , idx : usize) -> Self { debug_assert ! (idx < node . len ()) ; Handle { node , idx , _marker : PhantomData } } pub (super) fn left_edge (self) -> Handle < NodeRef < BorrowType , K , V , NodeType > , marker :: Edge > { unsafe { Handle :: new_edge (self . node , self . idx) } } pub (super) fn right_edge (self) -> Handle < NodeRef < BorrowType , K , V , NodeType > , marker :: Edge > { unsafe { Handle :: new_edge (self . node , self . idx + 1) } } }
};
}
