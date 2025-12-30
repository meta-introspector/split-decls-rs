// Generated macro for is_float (function)
macro_rules! Depcrate_operators_float_cmpis_float {
() => {
// Module: crate::operators::float_cmp
// Provides: {"is_float"}
// Dependencies: {}
fn is_float (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let value = & cx . typeck_results () . expr_ty (expr) . peel_refs () . kind () ; if let ty :: Array (arr_ty , _) = value { return matches ! (arr_ty . kind () , ty :: Float (_)) ; } matches ! (value , ty :: Float (_)) }
};
}
