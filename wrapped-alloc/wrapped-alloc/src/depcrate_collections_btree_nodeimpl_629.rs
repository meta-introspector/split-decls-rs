// Generated macro for impl_629 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_629 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_629"}
// Dependencies: {}
impl < BorrowType , K , V , NodeType > Handle < NodeRef < BorrowType , K , V , NodeType > , marker :: Edge > { # [doc = " Creates a new handle to an edge in `node`."] # [doc = " Unsafe because the caller must ensure that `idx <= node.len()`."] pub (super) unsafe fn new_edge (node : NodeRef < BorrowType , K , V , NodeType > , idx : usize) -> Self { debug_assert ! (idx <= node . len ()) ; Handle { node , idx , _marker : PhantomData } } pub (super) fn left_kv (self ,) -> Result < Handle < NodeRef < BorrowType , K , V , NodeType > , marker :: KV > , Self > { if self . idx > 0 { Ok (unsafe { Handle :: new_kv (self . node , self . idx - 1) }) } else { Err (self) } } pub (super) fn right_kv (self ,) -> Result < Handle < NodeRef < BorrowType , K , V , NodeType > , marker :: KV > , Self > { if self . idx < self . node . len () { Ok (unsafe { Handle :: new_kv (self . node , self . idx) }) } else { Err (self) } } }
};
}
