// Generated macro for InternalNode (struct)
macro_rules! Depcrate_collections_btree_nodeInternalNode {
() => {
// Module: crate::collections::btree::node
// Provides: {"InternalNode"}
// Dependencies: {}
# [doc = " The underlying representation of internal nodes. As with `LeafNode`s, these should be hidden"] # [doc = " behind `BoxedNode`s to prevent dropping uninitialized keys and values. Any pointer to an"] # [doc = " `InternalNode` can be directly cast to a pointer to the underlying `LeafNode` portion of the"] # [doc = " node, allowing code to act on leaf and internal nodes generically without having to even check"] # [doc = " which of the two a pointer is pointing at. This property is enabled by the use of `repr(C)`."] # [repr (C)] struct InternalNode < K , V > { data : LeafNode < K , V > , # [doc = " The pointers to the children of this node. `len + 1` of these are considered"] # [doc = " initialized and valid, except that near the end, while the tree is held"] # [doc = " through borrow type `Dying`, some of these pointers are dangling."] edges : [MaybeUninit < BoxedNode < K , V > > ; 2 * B] , }
};
}
