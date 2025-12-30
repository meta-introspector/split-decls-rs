// Generated macro for DominatorTreePreorder (struct)
macro_rules! Depcrate_dominator_treeDominatorTreePreorder {
() => {
// Module: crate::dominator_tree
// Provides: {"DominatorTreePreorder"}
// Dependencies: {}
# [doc = " Optional pre-order information that can be computed for a dominator tree."] # [doc = ""] # [doc = " This data structure is computed from a `DominatorTree` and provides:"] # [doc = ""] # [doc = " - A forward traversable dominator tree through the `children()` iterator."] # [doc = " - An ordering of blocks according to a dominator tree pre-order."] # [doc = " - Constant time dominance checks at the block granularity."] # [doc = ""] # [doc = " The information in this auxiliary data structure is not easy to update when the control flow"] # [doc = " graph changes, which is why it is kept separate."] pub struct DominatorTreePreorder { nodes : SecondaryMap < Block , ExtraNode > , stack : Vec < Block > , }
};
}
