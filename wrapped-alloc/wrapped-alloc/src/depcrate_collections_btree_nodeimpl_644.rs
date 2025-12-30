// Generated macro for impl_644 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_644 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_644"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: KV > { # [doc = " Splits the underlying node into three parts:"] # [doc = ""] # [doc = " - The node is truncated to only contain the key-value pairs to the left of"] # [doc = "   this handle."] # [doc = " - The key and value pointed to by this handle are extracted."] # [doc = " - All the key-value pairs to the right of this handle are put into a newly"] # [doc = "   allocated node."] pub (super) fn split < A : Allocator + Clone > (mut self , alloc : A ,) -> SplitResult < 'a , K , V , marker :: Leaf > { let mut new_node = LeafNode :: new (alloc) ; let kv = self . split_leaf_data (& mut new_node) ; let right = NodeRef :: from_new_leaf (new_node) ; SplitResult { left : self . node , kv , right } } # [doc = " Removes the key-value pair pointed to by this handle and returns it, along with the edge"] # [doc = " that the key-value pair collapsed into."] pub (super) fn remove (mut self ,) -> ((K , V) , Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge >) { let old_len = self . node . len () ; unsafe { let k = slice_remove (self . node . key_area_mut (.. old_len) , self . idx) ; let v = slice_remove (self . node . val_area_mut (.. old_len) , self . idx) ; * self . node . len_mut () = (old_len - 1) as u16 ; ((k , v) , self . left_edge ()) } } }
};
}
