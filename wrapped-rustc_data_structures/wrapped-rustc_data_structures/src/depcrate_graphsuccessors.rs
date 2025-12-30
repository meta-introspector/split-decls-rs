// Generated macro for Successors (trait)
macro_rules! Depcrate_graphSuccessors {
() => {
// Module: crate::graph
// Provides: {"Successors"}
// Dependencies: {}
pub trait Successors : DirectedGraph { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > ; }
};
}
