// Generated macro for expr_ty_has_significant_drop (function)
macro_rules! Depcrate_no_effectexpr_ty_has_significant_drop {
() => {
// Module: crate::no_effect
// Provides: {"expr_ty_has_significant_drop"}
// Dependencies: {}
# [doc = " Checks if dropping `expr` might have a visible side effect."] fn expr_ty_has_significant_drop (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let ty = cx . typeck_results () . expr_ty (expr) ; ty . has_significant_drop (cx . tcx , cx . typing_env ()) }
};
}
