// Generated macro for is_some_ident (function)
macro_rules! Depcrate_matches_manual_ok_erris_some_ident {
() => {
// Module: crate::matches::manual_ok_err
// Provides: {"is_some_ident"}
// Dependencies: {}
# [doc = " Check if `expr` contains `Some(ident)`, possibly as a block"] fn is_some_ident < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < '_ > , ident : & Ident , ty : Ty < 'tcx >) -> bool { if let Some (body_arg) = as_some_expr (cx , peel_blocks (expr)) && cx . typeck_results () . expr_ty (body_arg) == ty && let ExprKind :: Path (QPath :: Resolved (_ , Path { segments : [segment] , .. } ,)) = body_arg . kind { segment . ident . name == ident . name } else { false } }
};
}
