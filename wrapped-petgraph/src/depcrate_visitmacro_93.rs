// Generated macro for macro_93 (macro)
macro_rules! Depcrate_visitmacro_93 {
() => {
// Module: crate::visit
// Provides: {"macro_93"}
// Dependencies: {}
trait_template ! { # [doc = " Access to the edges of each node."] # [doc = ""] # [doc = " The edges are, depending on the graph’s edge type:"] # [doc = ""] # [doc = " - `Directed`: All edges from `a`."] # [doc = " - `Undirected`: All edges connected to `a`, with `a` being the source of each edge."] # [doc = ""] # [doc = " This is an extended version of the trait `IntoNeighbors`; the former"] # [doc = " only iterates over the target node identifiers, while this trait"] # [doc = " yields edge references (trait [`EdgeRef`][er])."] # [doc = ""] # [doc = " [er]: trait.EdgeRef.html"] pub trait IntoEdges : IntoEdgeReferences + IntoNeighbors { @ section type type Edges : Iterator < Item = Self :: EdgeRef >; @ section self fn edges (self , a : Self :: NodeId) -> Self :: Edges ; } }
};
}
