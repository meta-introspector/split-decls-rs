// Generated macro for macro_114 (macro)
macro_rules! Depcrate_visitmacro_114 {
() => {
// Module: crate::visit
// Provides: {"macro_114"}
// Dependencies: {}
trait_template ! { # [doc = " The graph’s `NodeId`s map to indices"] # [allow (clippy :: needless_arbitrary_self_type)] pub trait NodeIndexable : GraphBase { @ section self # [doc = " Return an upper bound of the node indices in the graph"] # [doc = " (suitable for the size of a bitmap)."] fn node_bound (self : & Self) -> usize ; # [doc = " Convert `a` to an integer index."] # [track_caller] fn to_index (self : & Self , a : Self :: NodeId) -> usize ; # [doc = " Convert `i` to a node index. `i` must be a valid value in the graph."] # [track_caller] fn from_index (self : & Self , i : usize) -> Self :: NodeId ; } }
};
}
