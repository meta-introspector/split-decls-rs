// Generated macro for impl_151 (impl)
macro_rules! Depcrate_subgraph_freeimpl_151 {
() => {
// Module: crate::subgraph_free
// Provides: {"impl_151"}
// Dependencies: {}
impl < A > FromIterator < Stmt < A > > for StmtList < A > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = Stmt < A > > , { Self { stmts : iter . into_iter () . collect () , } } }
};
}
