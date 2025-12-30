// Generated macro for impl_941 (impl)
macro_rules! Depcrate_graph_implimpl_941 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_941"}
// Dependencies: {}
impl < E , Ix : IndexType > Edge < E , Ix > { # [doc = " Accessor for data structure internals: the next edge for the given direction."] pub fn next_edge (& self , dir : Direction) -> EdgeIndex < Ix > { self . next [dir . index ()] } # [doc = " Return the source node index."] pub fn source (& self) -> NodeIndex < Ix > { self . node [0] } # [doc = " Return the target node index."] pub fn target (& self) -> NodeIndex < Ix > { self . node [1] } }
};
}
