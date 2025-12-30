// Generated macro for check_int_ty_and_feature (function)
macro_rules! Depcrate_operators_manual_div_ceilcheck_int_ty_and_feature {
() => {
// Module: crate::operators::manual_div_ceil
// Provides: {"check_int_ty_and_feature"}
// Dependencies: {}
fn check_int_ty_and_feature (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let expr_ty = cx . typeck_results () . expr_ty (expr) ; match expr_ty . peel_refs () . kind () { ty :: Uint (_) => true , ty :: Int (_) => cx . tcx . features () . enabled (sym :: int_roundings) , _ => false , } }
};
}
