// Generated macro for impl_925 (impl)
macro_rules! Depcrate_graph_implimpl_925 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_925"}
// Dependencies: {}
impl < Ix : IndexType > NodeIndex < Ix > { # [inline] pub fn new (x : usize) -> Self { NodeIndex (IndexType :: new (x)) } # [inline] pub fn index (self) -> usize { self . 0 . index () } # [inline] pub fn end () -> Self { NodeIndex (IndexType :: max ()) } fn _into_edge (self) -> EdgeIndex < Ix > { EdgeIndex (self . 0) } }
};
}
