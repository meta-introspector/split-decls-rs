// Generated macro for try_parse_op_call (function)
macro_rules! Depcrate_set_contains_or_inserttry_parse_op_call {
() => {
// Module: crate::set_contains_or_insert
// Provides: {"try_parse_op_call"}
// Dependencies: {}
fn try_parse_op_call < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , symbol : Symbol ,) -> Option < (OpExpr < 'tcx > , Symbol) > { let expr = peel_hir_expr_while (expr , | e | { if let ExprKind :: Unary (UnOp :: Not , e) = e . kind { Some (e) } else { None } }) ; if let ExprKind :: MethodCall (path , receiver , [value] , span) = expr . kind { let value = value . peel_borrows () ; let value = peel_hir_expr_while (value , | e | { if let ExprKind :: Unary (UnOp :: Deref , e) = e . kind { Some (e) } else { None } }) ; let receiver = receiver . peel_borrows () ; let receiver_ty = cx . typeck_results () . expr_ty (receiver) . peel_refs () ; if value . span . eq_ctxt (expr . span) && path . ident . name == symbol { for sym in & [sym :: HashSet , sym :: BTreeSet] { if receiver_ty . is_diag_item (cx , * sym) { return Some ((OpExpr { receiver , value , span } , * sym)) ; } } } } None }
};
}
