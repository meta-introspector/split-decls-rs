// Generated macro for LeafNode (struct)
macro_rules! Depcrate_collections_btree_nodeLeafNode {
() => {
// Module: crate::collections::btree::node
// Provides: {"LeafNode"}
// Dependencies: {}
# [doc = " The underlying representation of leaf nodes and part of the representation of internal nodes."] struct LeafNode < K , V > { # [doc = " We want to be covariant in `K` and `V`."] parent : Option < NonNull < InternalNode < K , V > > > , # [doc = " This node's index into the parent node's `edges` array."] # [doc = " `*node.parent.edges[node.parent_idx]` should be the same thing as `node`."] # [doc = " This is only guaranteed to be initialized when `parent` is non-null."] parent_idx : MaybeUninit < u16 > , # [doc = " The number of keys and values this node stores."] len : u16 , # [doc = " The arrays storing the actual data of the node. Only the first `len` elements of each"] # [doc = " array are initialized and valid."] keys : [MaybeUninit < K > ; CAPACITY] , vals : [MaybeUninit < V > ; CAPACITY] , }
};
}
