// Generated macro for expr_is_erased_ref (function)
macro_rules! Depcrate_operators_identity_opexpr_is_erased_ref {
() => {
// Module: crate::operators::identity_op
// Provides: {"expr_is_erased_ref"}
// Dependencies: {}
fn expr_is_erased_ref (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { match cx . typeck_results () . expr_ty (expr) . kind () { ty :: Ref (r , ..) => r . is_erased () , _ => false , } }
};
}
