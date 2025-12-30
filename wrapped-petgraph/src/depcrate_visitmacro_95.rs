// Generated macro for macro_95 (macro)
macro_rules! Depcrate_visitmacro_95 {
() => {
// Module: crate::visit
// Provides: {"macro_95"}
// Dependencies: {}
trait_template ! { # [doc = " Access to all edges of each node, in the specified direction."] # [doc = ""] # [doc = " The edges are, depending on the direction and the graph’s edge type:"] # [doc = ""] # [doc = ""] # [doc = " - `Directed`, `Outgoing`: All edges from `a`."] # [doc = " - `Directed`, `Incoming`: All edges to `a`."] # [doc = " - `Undirected`, `Outgoing`: All edges connected to `a`, with `a` being the source of each edge."] # [doc = " - `Undirected`, `Incoming`: All edges connected to `a`, with `a` being the target of each edge."] # [doc = ""] # [doc = " This is an extended version of the trait `IntoNeighborsDirected`; the former"] # [doc = " only iterates over the target node identifiers, while this trait"] # [doc = " yields edge references (trait [`EdgeRef`][er])."] # [doc = ""] # [doc = " [er]: trait.EdgeRef.html"] pub trait IntoEdgesDirected : IntoEdges + IntoNeighborsDirected { @ section type type EdgesDirected : Iterator < Item = Self :: EdgeRef >; @ section self fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected ; } }
};
}
