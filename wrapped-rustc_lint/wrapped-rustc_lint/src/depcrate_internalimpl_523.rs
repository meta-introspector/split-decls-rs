// Generated macro for impl_523 (impl)
macro_rules! Depcrate_internalimpl_523 {
() => {
// Module: crate::internal
// Provides: {"impl_523"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SpanUseEqCtxt { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & hir :: Expr < '_ >) { if let hir :: ExprKind :: Binary (hir :: BinOp { node : hir :: BinOpKind :: Eq | hir :: BinOpKind :: Ne , .. } , lhs , rhs ,) = expr . kind { if is_span_ctxt_call (cx , lhs) && is_span_ctxt_call (cx , rhs) { cx . emit_span_lint (SPAN_USE_EQ_CTXT , expr . span , SpanUseEqCtxtDiag) ; } } } }
};
}
