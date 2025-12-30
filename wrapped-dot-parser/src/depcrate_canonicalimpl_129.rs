// Generated macro for impl_129 (impl)
macro_rules! Depcrate_canonicalimpl_129 {
() => {
// Module: crate::canonical
// Provides: {"impl_129"}
// Dependencies: {}
impl < A , I > From < (I , & mut NodeSet < A >) > for EdgeSet < A > where I : IntoIterator < Item = EdgeStmt < A > > , A : Clone , { fn from (tuple : (I , & mut NodeSet < A >)) -> Self { let (stmts , nodes) = tuple ; let mut set = EdgeSet :: empty () ; for stmt in stmts { set += (stmt , & mut * nodes) . into () ; } set } }
};
}
