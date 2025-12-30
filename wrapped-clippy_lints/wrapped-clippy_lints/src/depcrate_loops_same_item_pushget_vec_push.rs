// Generated macro for get_vec_push (function)
macro_rules! Depcrate_loops_same_item_pushget_vec_push {
() => {
// Module: crate::loops::same_item_push
// Provides: {"get_vec_push"}
// Dependencies: {}
fn get_vec_push < 'tcx > (cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < '_ > ,) -> Option < (& 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx > , SyntaxContext) > { if let StmtKind :: Semi (semi_stmt) = & stmt . kind && let ExprKind :: MethodCall (path , self_expr , [pushed_item] , _) = & semi_stmt . kind && path . ident . name == sym :: push && cx . typeck_results () . expr_ty (self_expr) . is_diag_item (cx , sym :: Vec) { return Some ((self_expr , pushed_item , semi_stmt . span . ctxt ())) ; } None }
};
}
