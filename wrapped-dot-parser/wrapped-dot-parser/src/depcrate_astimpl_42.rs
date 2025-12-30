// Generated macro for impl_42 (impl)
macro_rules! Depcrate_astimpl_42 {
() => {
// Module: crate::ast
// Provides: {"impl_42"}
// Dependencies: {}
impl < A > IntoIterator for StmtList < A > { type Item = Stmt < A > ; type IntoIter = std :: vec :: IntoIter < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . stmts . into_iter () } }
};
}
