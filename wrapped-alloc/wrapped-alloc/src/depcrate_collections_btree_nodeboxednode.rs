// Generated macro for BoxedNode (type)
macro_rules! Depcrate_collections_btree_nodeBoxedNode {
() => {
// Module: crate::collections::btree::node
// Provides: {"BoxedNode"}
// Dependencies: {}
# [doc = " A managed, non-null pointer to a node. This is either an owned pointer to"] # [doc = " `LeafNode<K, V>` or an owned pointer to `InternalNode<K, V>`."] # [doc = ""] # [doc = " However, `BoxedNode` contains no information as to which of the two types"] # [doc = " of nodes it actually contains, and, partially due to this lack of information,"] # [doc = " is not a separate type and has no destructor."] type BoxedNode < K , V > = NonNull < LeafNode < K , V > > ;
};
}
