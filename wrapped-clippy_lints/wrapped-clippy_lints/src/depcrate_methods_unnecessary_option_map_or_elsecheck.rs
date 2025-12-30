// Generated macro for check (function)
macro_rules! Depcrate_methods_unnecessary_option_map_or_elsecheck {
() => {
// Module: crate::methods::unnecessary_option_map_or_else
// Provides: {"check"}
// Dependencies: {}
# [doc = " lint use of `_.map_or_else(|err| err, |n| n)` for `Option`s."] pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , def_arg : & Expr < '_ > , map_arg : & Expr < '_ >) { if ! cx . typeck_results () . expr_ty (recv) . is_diag_item (cx , sym :: Option) { return ; } match map_arg . kind { ExprKind :: Closure (& Closure { body , .. }) => { handle_closure (cx , expr , recv , def_arg , body) ; } , ExprKind :: Path (qpath) => { let res = cx . qpath_res (& qpath , map_arg . hir_id) ; match res { Res :: Local (hir_id) => { if let Some (init_expr) = find_binding_init (cx , hir_id) { let origin = expr_or_init (cx , init_expr) ; if let ExprKind :: Closure (& Closure { body , .. }) = origin . kind { handle_closure (cx , expr , recv , def_arg , body) ; } } } , Res :: Def (DefKind :: Fn , def_id) => { if let Some (local_def_id) = def_id . as_local () && let Some (body) = cx . tcx . hir_maybe_body_owned_by (local_def_id) { handle_fn_body (cx , expr , recv , def_arg , body) ; } } , _ => () , } } , _ => () , } }
};
}
