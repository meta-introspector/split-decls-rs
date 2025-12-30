// Generated macro for impl_926 (impl)
macro_rules! Depcrate_graph_implimpl_926 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_926"}
// Dependencies: {}
unsafe impl < Ix : IndexType > IndexType for NodeIndex < Ix > { fn index (& self) -> usize { self . 0 . index () } fn new (x : usize) -> Self { NodeIndex :: new (x) } fn max () -> Self { NodeIndex (< Ix as IndexType > :: max ()) } }
};
}
