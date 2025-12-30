// Generated macro for check (function)
macro_rules! Depcrate_methods_unnecessary_result_map_or_elsecheck {
() => {
// Module: crate::methods::unnecessary_result_map_or_else
// Provides: {"check"}
// Dependencies: {}
# [doc = " lint use of `_.map_or_else(|err| err, |n| n)` for `Result`s."] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , recv : & 'tcx Expr < '_ > , def_arg : & 'tcx Expr < '_ > , map_arg : & 'tcx Expr < '_ > ,) { if cx . typeck_results () . expr_ty (recv) . is_diag_item (cx , sym :: Result) && let ExprKind :: Closure (& Closure { body , .. }) = map_arg . kind && let body = cx . tcx . hir_body (body) && let Some (first_param) = body . params . first () { let body_expr = peel_blocks (body . value) ; match body_expr . kind { ExprKind :: Path (qpath) => { handle_qpath (cx , expr , recv , def_arg , first_param . pat . hir_id , qpath) ; } , ExprKind :: Block (block , _) => { if let Some (block_expr) = block . expr && let Some (last_chain_binding_id) = get_last_chain_binding_hir_id (first_param . pat . hir_id , block . stmts) && let ExprKind :: Path (qpath) = block_expr . kind { handle_qpath (cx , expr , recv , def_arg , last_chain_binding_id , qpath) ; } } , _ => { } , } } }
};
}
