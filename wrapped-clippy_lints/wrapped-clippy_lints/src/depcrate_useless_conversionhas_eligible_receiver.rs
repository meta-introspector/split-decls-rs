// Generated macro for has_eligible_receiver (function)
macro_rules! Depcrate_useless_conversionhas_eligible_receiver {
() => {
// Module: crate::useless_conversion
// Provides: {"has_eligible_receiver"}
// Dependencies: {}
fn has_eligible_receiver (cx : & LateContext < '_ > , recv : & Expr < '_ > , expr : & Expr < '_ >) -> bool { if cx . ty_based_def (expr) . opt_parent (cx) . is_impl (cx) { matches ! (cx . typeck_results () . expr_ty (recv) . opt_diag_name (cx) , Some (sym :: Option | sym :: Result | sym :: ControlFlow)) } else { cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) } }
};
}
