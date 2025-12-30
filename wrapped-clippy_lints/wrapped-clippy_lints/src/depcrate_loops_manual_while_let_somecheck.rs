// Generated macro for check (function)
macro_rules! Depcrate_loops_manual_while_let_somecheck {
() => {
// Module: crate::loops::manual_while_let_some
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , full_cond : & 'tcx Expr < '_ > , body : & 'tcx Expr < '_ > , loop_span : Span) { if let ExprKind :: Unary (UnOp :: Not , cond) = full_cond . kind && let ExprKind :: MethodCall (_ , is_empty_recv , _ , _) = cond . kind && match_method_call :: < 0 > (cx , cond , sym :: vec_is_empty) && let ExprKind :: Block (body , _) = body . kind && let Some (stmt) = body . stmts . first () { check_local (cx , stmt , is_empty_recv , loop_span) ; check_call_arguments (cx , stmt , is_empty_recv , loop_span) ; } }
};
}
