// Generated macro for is_single_call_in_arm (function)
macro_rules! Depcrate_drop_forget_refis_single_call_in_arm {
() => {
// Module: crate::drop_forget_ref
// Provides: {"is_single_call_in_arm"}
// Dependencies: {}
fn is_single_call_in_arm < 'tcx > (cx : & LateContext < 'tcx > , arg : & 'tcx Expr < '_ > , drop_expr : & 'tcx Expr < '_ >) -> bool { if matches ! (arg . kind , ExprKind :: Call (..) | ExprKind :: MethodCall (..)) && let Node :: Arm (Arm { body , .. }) = cx . tcx . parent_hir_node (drop_expr . hir_id) { return body . hir_id == drop_expr . hir_id ; } false }
};
}
