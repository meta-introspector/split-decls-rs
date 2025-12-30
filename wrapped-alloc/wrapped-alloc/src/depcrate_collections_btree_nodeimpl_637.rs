// Generated macro for impl_637 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_637 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_637"}
// Dependencies: {}
impl < BorrowType : marker :: BorrowType , K , V > Handle < NodeRef < BorrowType , K , V , marker :: Internal > , marker :: Edge > { # [doc = " Finds the node pointed to by this edge."] # [doc = ""] # [doc = " The method name assumes you picture trees with the root node on top."] # [doc = ""] # [doc = " `edge.descend().ascend().unwrap()` and `node.ascend().unwrap().descend()` should"] # [doc = " both, upon success, do nothing."] pub (super) fn descend (self) -> NodeRef < BorrowType , K , V , marker :: LeafOrInternal > { const { assert ! (BorrowType :: TRAVERSAL_PERMIT) ; } let parent_ptr = NodeRef :: as_internal_ptr (& self . node) ; let node = unsafe { (* parent_ptr) . edges . get_unchecked (self . idx) . assume_init_read () } ; NodeRef { node , height : self . node . height - 1 , _marker : PhantomData } } }
};
}
