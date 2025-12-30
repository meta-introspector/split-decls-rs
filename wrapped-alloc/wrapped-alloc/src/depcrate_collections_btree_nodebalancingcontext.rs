// Generated macro for BalancingContext (struct)
macro_rules! Depcrate_collections_btree_nodeBalancingContext {
() => {
// Module: crate::collections::btree::node
// Provides: {"BalancingContext"}
// Dependencies: {}
# [doc = " Represents a session for evaluating and performing a balancing operation"] # [doc = " around an internal key-value pair."] pub (super) struct BalancingContext < 'a , K , V > { parent : Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > , marker :: KV > , left_child : NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > , right_child : NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > , }
};
}
