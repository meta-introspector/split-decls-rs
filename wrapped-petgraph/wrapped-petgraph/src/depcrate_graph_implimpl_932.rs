// Generated macro for impl_932 (impl)
macro_rules! Depcrate_graph_implimpl_932 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_932"}
// Dependencies: {}
impl < Ix : IndexType > EdgeIndex < Ix > { # [inline] pub fn new (x : usize) -> Self { EdgeIndex (IndexType :: new (x)) } # [inline] pub fn index (self) -> usize { self . 0 . index () } # [doc = " An invalid `EdgeIndex` used to denote absence of an edge, for example"] # [doc = " to end an adjacency list."] # [inline] pub fn end () -> Self { EdgeIndex (IndexType :: max ()) } fn _into_node (self) -> NodeIndex < Ix > { NodeIndex (self . 0) } }
};
}
