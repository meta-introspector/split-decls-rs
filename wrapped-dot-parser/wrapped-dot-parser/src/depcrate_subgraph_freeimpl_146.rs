// Generated macro for impl_146 (impl)
macro_rules! Depcrate_subgraph_freeimpl_146 {
() => {
// Module: crate::subgraph_free
// Provides: {"impl_146"}
// Dependencies: {}
impl < A > From < crate :: ast :: Graph < A > > for Graph < A > where A : Clone , { fn from (g : crate :: ast :: Graph < A >) -> Self { Graph { strict : g . strict , is_digraph : g . is_digraph , name : g . name , stmts : g . stmts . into () , } } }
};
}
