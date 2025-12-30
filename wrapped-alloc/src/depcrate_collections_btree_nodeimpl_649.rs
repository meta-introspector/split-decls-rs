// Generated macro for impl_649 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_649 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_649"}
// Dependencies: {}
impl < 'a , K , V > BalancingContext < 'a , K , V > { pub (super) fn left_child_len (& self) -> usize { self . left_child . len () } pub (super) fn right_child_len (& self) -> usize { self . right_child . len () } pub (super) fn into_left_child (self) -> NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > { self . left_child } pub (super) fn into_right_child (self) -> NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > { self . right_child } # [doc = " Returns whether merging is possible, i.e., whether there is enough room"] # [doc = " in a node to combine the central KV with both adjacent child nodes."] pub (super) fn can_merge (& self) -> bool { self . left_child . len () + 1 + self . right_child . len () <= CAPACITY } }
};
}
