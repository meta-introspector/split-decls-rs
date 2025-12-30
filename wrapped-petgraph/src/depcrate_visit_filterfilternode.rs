// Generated macro for FilterNode (trait)
macro_rules! Depcrate_visit_filterFilterNode {
() => {
// Module: crate::visit::filter
// Provides: {"FilterNode"}
// Dependencies: {}
# [doc = " A graph filter for nodes."] pub trait FilterNode < N > { # [doc = " Return true to have the node be part of the graph"] fn include_node (& self , node : N) -> bool ; }
};
}
