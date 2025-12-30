// Generated macro for impl_43 (impl)
macro_rules! Depcrate_astimpl_43 {
() => {
// Module: crate::ast
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a , A > IntoIterator for & 'a StmtList < A > { type Item = & 'a Stmt < A > ; type IntoIter = std :: slice :: Iter < 'a , Stmt < A > > ; fn into_iter (self) -> Self :: IntoIter { self . stmts . iter () } }
};
}
