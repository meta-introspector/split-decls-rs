// Generated macro for check_map (function)
macro_rules! Depcrate_methods_manual_is_variant_andcheck_map {
() => {
// Module: crate::methods::manual_is_variant_and
// Provides: {"check_map"}
// Dependencies: {}
pub (super) fn check_map (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let Some (parent_expr) = get_parent_expr (cx , expr) && let ExprKind :: Binary (op , left , right) = parent_expr . kind && op . span . eq_ctxt (expr . span) && let Ok (op) = Op :: try_from (op . node) { for (expr1 , expr2) in [(left , right) , (right , left)] { for flavor in [Flavor :: Option , Flavor :: Result] { if let ExprKind :: Call (call , [arg]) = expr1 . kind && let ExprKind :: Lit (lit) = arg . kind && let LitKind :: Bool (bool_cst) = lit . node && let ExprKind :: Path (QPath :: Resolved (_ , path)) = call . kind && let Res :: Def (DefKind :: Ctor (CtorOf :: Variant , CtorKind :: Fn) , _) = path . res && let ty = cx . typeck_results () . expr_ty (expr1) && let ty :: Adt (adt , args) = ty . kind () && cx . tcx . is_diagnostic_item (flavor . symbol () , adt . did ()) && args . type_at (0) . is_bool () && let ExprKind :: MethodCall (_ , recv , [map_expr] , _) = expr2 . kind && cx . typeck_results () . expr_ty (recv) . is_diag_item (cx , flavor . symbol ()) && let Ok (map_func) = MapFunc :: try_from (map_expr) { return emit_lint (cx , parent_expr . span , op , flavor , bool_cst , map_func , recv) ; } } } } }
};
}
