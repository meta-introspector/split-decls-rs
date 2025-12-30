// Generated macro for impl_93 (impl)
macro_rules! Depcrate_astimpl_93 {
() => {
// Module: crate::ast
// Provides: {"impl_93"}
// Dependencies: {}
impl < A > Subgraph < A > { fn filter_map_attr < B > (self , f : & dyn Fn (A) -> Option < B >) -> Subgraph < B > { Subgraph { id : self . id , stmts : self . stmts . into_iter () . map (| stmt | stmt . filter_map_attr (f)) . collect () , } } # [doc = " Extract a subgraph as a standalone graph."] pub (crate) fn into_graph (self , strict : bool , is_digraph : bool) -> Graph < A > { Graph { strict , is_digraph , name : self . id . map (String :: from) , stmts : self . stmts , } } fn get_node_ids (& self) -> HashSet < NodeID > { self . stmts . get_node_ids () } }
};
}
