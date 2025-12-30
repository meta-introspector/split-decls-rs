// Generated macro for impl_150 (impl)
macro_rules! Depcrate_subgraph_freeimpl_150 {
() => {
// Module: crate::subgraph_free
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'a , A > IntoIterator for & 'a StmtList < A > { type Item = & 'a Stmt < A > ; type IntoIter = std :: slice :: Iter < 'a , Stmt < A > > ; fn into_iter (self) -> Self :: IntoIter { self . stmts . iter () } }
};
}
