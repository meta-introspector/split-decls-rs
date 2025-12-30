// Generated macro for FilterEdge (trait)
macro_rules! Depcrate_visit_filterFilterEdge {
() => {
// Module: crate::visit::filter
// Provides: {"FilterEdge"}
// Dependencies: {}
# [doc = " A graph filter for edges"] pub trait FilterEdge < Edge > { # [doc = " Return true to have the edge be part of the graph"] fn include_edge (& self , edge : Edge) -> bool ; }
};
}
