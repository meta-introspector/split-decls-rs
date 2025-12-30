// Generated macro for edges_walker_mut (function)
macro_rules! Depcrate_graph_impledges_walker_mut {
() => {
// Module: crate::graph_impl
// Provides: {"edges_walker_mut"}
// Dependencies: {}
fn edges_walker_mut < E , Ix > (edges : & mut [Edge < E , Ix >] , next : EdgeIndex < Ix > , dir : Direction ,) -> EdgesWalkerMut < '_ , E , Ix > where Ix : IndexType , { EdgesWalkerMut { edges , next , dir } }
};
}
