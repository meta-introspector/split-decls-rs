// Generated macro for expr_type_is_certain (function)
macro_rules! Depcrate_ty_type_certaintyexpr_type_is_certain {
() => {
// Module: crate::ty::type_certainty
// Provides: {"expr_type_is_certain"}
// Dependencies: {}
pub fn expr_type_is_certain (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { expr_type_certainty (cx , expr , false) . is_certain () }
};
}
