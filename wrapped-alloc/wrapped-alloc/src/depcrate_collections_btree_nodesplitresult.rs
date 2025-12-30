// Generated macro for SplitResult (struct)
macro_rules! Depcrate_collections_btree_nodeSplitResult {
() => {
// Module: crate::collections::btree::node
// Provides: {"SplitResult"}
// Dependencies: {}
# [doc = " Result of insertion, when a node needed to expand beyond its capacity."] pub (super) struct SplitResult < 'a , K , V , NodeType > { pub left : NodeRef < marker :: Mut < 'a > , K , V , NodeType > , pub kv : (K , V) , pub right : NodeRef < marker :: Owned , K , V , NodeType > , }
};
}
