// Generated macro for get_ty_sugg (function)
macro_rules! Depcrate_manual_is_ascii_checkget_ty_sugg {
() => {
// Module: crate::manual_is_ascii_check
// Provides: {"get_ty_sugg"}
// Dependencies: {}
fn get_ty_sugg < 'tcx > (cx : & LateContext < 'tcx > , arg : & Expr < '_ >) -> Option < (Span , Ty < 'tcx >) > { let local_hid = arg . res_local_id () ? ; if let Node :: Param (Param { ty_span , span , .. }) = cx . tcx . parent_hir_node (local_hid) && ty_span == span { let arg_type = cx . typeck_results () . expr_ty (arg) ; return Some ((* ty_span , arg_type)) ; } None }
};
}
