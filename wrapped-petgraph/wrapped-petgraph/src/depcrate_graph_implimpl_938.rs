// Generated macro for impl_938 (impl)
macro_rules! Depcrate_graph_implimpl_938 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_938"}
// Dependencies: {}
impl < N , Ix : IndexType > Node < N , Ix > { # [doc = " Accessor for data structure internals: the first edge in the given direction."] pub fn next_edge (& self , dir : Direction) -> EdgeIndex < Ix > { self . next [dir . index ()] } }
};
}
