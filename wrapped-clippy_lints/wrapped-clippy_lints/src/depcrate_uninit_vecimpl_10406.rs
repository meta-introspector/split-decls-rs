// Generated macro for impl_10406 (impl)
macro_rules! Depcrate_uninit_vecimpl_10406 {
() => {
// Module: crate::uninit_vec
// Provides: {"impl_10406"}
// Dependencies: {}
impl < 'tcx > VecLocation < 'tcx > { pub fn eq_expr (self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> bool { match self { VecLocation :: Local (hir_id) => path_to_local_id (expr , hir_id) , VecLocation :: Expr (self_expr) => SpanlessEq :: new (cx) . eq_expr (self_expr , expr) , } } }
};
}
