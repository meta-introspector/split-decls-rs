// Generated macro for impl_10755 (impl)
macro_rules! Depcrate_uninit_vecimpl_10755 {
() => {
// Module: crate::uninit_vec
// Provides: {"impl_10755"}
// Dependencies: {}
impl < 'tcx > VecLocation < 'tcx > { pub fn eq_expr (self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> bool { match self { VecLocation :: Local (hir_id) => expr . res_local_id () == Some (hir_id) , VecLocation :: Expr (self_expr) => SpanlessEq :: new (cx) . eq_expr (self_expr , expr) , } } }
};
}
