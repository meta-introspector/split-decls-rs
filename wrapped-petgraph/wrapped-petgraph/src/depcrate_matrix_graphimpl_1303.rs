// Generated macro for impl_1303 (impl)
macro_rules! Depcrate_matrix_graphimpl_1303 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1303"}
// Dependencies: {}
impl < N , E , S : BuildHasher , Null : Nullable < Wrapped = E > , Ix : IndexType > MatrixGraph < N , E , S , Directed , Null , Ix > { # [doc = " Return an iterator of all neighbors that have an edge between them and"] # [doc = " `a`, in the specified direction."] # [doc = " If the graph's edges are undirected, this is equivalent to *.neighbors(a)*."] # [doc = ""] # [doc = " - `Outgoing`: All edges from `a`."] # [doc = " - `Incoming`: All edges to `a`."] # [doc = ""] # [doc = " Produces an empty iterator if the node doesn't exist.<br>"] # [doc = " Iterator element type is [`NodeIndex<Ix>`](../graph/struct.NodeIndex.html)."] pub fn neighbors_directed (& self , a : NodeIndex < Ix > , d : Direction ,) -> Neighbors < '_ , Directed , Null , Ix > { if d == Outgoing { self . neighbors (a) } else { Neighbors (Edges :: on_rows (a . index () , & self . node_adjacencies , self . node_capacity ,)) } } # [doc = " Return an iterator of all edges of `a`, in the specified direction."] # [doc = ""] # [doc = " - `Outgoing`: All edges from `a`."] # [doc = " - `Incoming`: All edges to `a`."] # [doc = ""] # [doc = " Produces an empty iterator if the node `a` doesn't exist.<br>"] # [doc = " Iterator element type is `(NodeIndex<Ix>, NodeIndex<Ix>, &E)`."] pub fn edges_directed (& self , a : NodeIndex < Ix > , d : Direction) -> Edges < '_ , Directed , Null , Ix > { if d == Outgoing { self . edges (a) } else { Edges :: on_rows (a . index () , & self . node_adjacencies , self . node_capacity) } } }
};
}
