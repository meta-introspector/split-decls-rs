// Generated macro for impl_44 (impl)
macro_rules! Depcrate_astimpl_44 {
() => {
// Module: crate::ast
// Provides: {"impl_44"}
// Dependencies: {}
impl < A > FromIterator < Stmt < A > > for StmtList < A > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = Stmt < A > > , { Self { stmts : iter . into_iter () . collect () , } } }
};
}
