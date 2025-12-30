// Generated macro for is_array (function)
macro_rules! Depcrate_operators_float_cmpis_array {
() => {
// Module: crate::operators::float_cmp
// Provides: {"is_array"}
// Dependencies: {}
fn is_array (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { matches ! (& cx . typeck_results () . expr_ty (expr) . peel_refs () . kind () , ty :: Array (_ , _)) }
};
}
