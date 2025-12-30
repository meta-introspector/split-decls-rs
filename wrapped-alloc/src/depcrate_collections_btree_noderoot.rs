// Generated macro for Root (type)
macro_rules! Depcrate_collections_btree_nodeRoot {
() => {
// Module: crate::collections::btree::node
// Provides: {"Root"}
// Dependencies: {}
# [doc = " The root node of an owned tree."] # [doc = ""] # [doc = " Note that this does not have a destructor, and must be cleaned up manually."] pub (super) type Root < K , V > = NodeRef < marker :: Owned , K , V , marker :: LeafOrInternal > ;
};
}
