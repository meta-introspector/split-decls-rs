// Generated macro for macro_112 (macro)
macro_rules! Depcrate_visitmacro_112 {
() => {
// Module: crate::visit
// Provides: {"macro_112"}
// Dependencies: {}
trait_template ! { # [doc = " Edge kind property (directed or undirected edges)"] pub trait GraphProp : GraphBase { @ section type # [doc = " The kind of edges in the graph."] type EdgeType : EdgeType ; @ section nodelegate fn is_directed (& self) -> bool { < Self :: EdgeType >:: is_directed () } } }
};
}
