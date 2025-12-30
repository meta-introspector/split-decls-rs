// Generated macro for impl_645 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_645 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_645"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > , marker :: KV > { # [doc = " Splits the underlying node into three parts:"] # [doc = ""] # [doc = " - The node is truncated to only contain the edges and key-value pairs to the"] # [doc = "   left of this handle."] # [doc = " - The key and value pointed to by this handle are extracted."] # [doc = " - All the edges and key-value pairs to the right of this handle are put into"] # [doc = "   a newly allocated node."] pub (super) fn split < A : Allocator + Clone > (mut self , alloc : A ,) -> SplitResult < 'a , K , V , marker :: Internal > { let old_len = self . node . len () ; unsafe { let mut new_node = InternalNode :: new (alloc) ; let kv = self . split_leaf_data (& mut new_node . data) ; let new_len = usize :: from (new_node . data . len) ; move_to_slice (self . node . edge_area_mut (self . idx + 1 .. old_len + 1) , & mut new_node . edges [.. new_len + 1] ,) ; let height = self . node . height ; let right = NodeRef :: from_new_internal (new_node , height) ; SplitResult { left : self . node , kv , right } } } }
};
}
