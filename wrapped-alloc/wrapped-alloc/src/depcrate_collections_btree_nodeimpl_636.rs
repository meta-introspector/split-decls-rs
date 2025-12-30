// Generated macro for impl_636 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_636 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_636"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Inserts a new key-value pair between the key-value pairs to the right and left of"] # [doc = " this edge. This method splits the node if there isn't enough room, and tries to"] # [doc = " insert the split off portion into the parent node recursively, until the root is reached."] # [doc = ""] # [doc = " If the returned result is some `SplitResult`, the `left` field will be the root node."] # [doc = " The returned pointer points to the inserted value, which in the case of `SplitResult`"] # [doc = " is in the `left` or `right` tree."] pub (super) fn insert_recursing < A : Allocator + Clone > (self , key : K , value : V , alloc : A , split_root : impl FnOnce (SplitResult < 'a , K , V , marker :: LeafOrInternal >) ,) -> Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: KV > { let (mut split , handle) = match self . insert (key , value , alloc . clone ()) { (None , handle) => return unsafe { handle . awaken () } , (Some (split) , handle) => (split . forget_node_type () , handle) , } ; loop { split = match split . left . ascend () { Ok (parent) => { match parent . insert (split . kv . 0 , split . kv . 1 , split . right , alloc . clone ()) { None => return unsafe { handle . awaken () } , Some (split) => split . forget_node_type () , } } Err (root) => { split_root (SplitResult { left : root , .. split }) ; return unsafe { handle . awaken () } ; } } ; } } }
};
}
