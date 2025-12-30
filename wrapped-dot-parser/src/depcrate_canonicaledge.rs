// Generated macro for Edge (struct)
macro_rules! Depcrate_canonicalEdge {
() => {
// Module: crate::canonical
// Provides: {"Edge"}
// Dependencies: {}
# [doc = " A single edge of the graph."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Edge < A > { # [doc = " The name of the origin of the edge."] pub from : String , # [doc = " The name of the destination of the edge."] pub to : String , # [doc = " A list of attributes that apply to this specific edge."] pub attr : AList < A > , }
};
}
