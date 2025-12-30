// Generated macro for different_types (function)
macro_rules! Depcrate_operators_erasing_opdifferent_types {
() => {
// Module: crate::operators::erasing_op
// Provides: {"different_types"}
// Dependencies: {}
fn different_types (tck : & TypeckResults < '_ > , input : & Expr < '_ > , output : & Expr < '_ >) -> bool { let input_ty = tck . expr_ty (input) . peel_refs () ; let output_ty = tck . expr_ty (output) . peel_refs () ; ! same_type_modulo_regions (input_ty , output_ty) }
};
}
