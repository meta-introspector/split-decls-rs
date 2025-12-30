// Generated macro for is_side_effect_free (function)
macro_rules! Depcrate_implicit_saturating_subis_side_effect_free {
() => {
// Module: crate::implicit_saturating_sub
// Provides: {"is_side_effect_free"}
// Dependencies: {}
fn is_side_effect_free (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { eq_expr_value (cx , expr , expr) }
};
}
