// Generated macro for is_allowed (function)
macro_rules! Depcrate_operators_identity_opis_allowed {
() => {
// Module: crate::operators::identity_op
// Provides: {"is_allowed"}
// Dependencies: {}
fn is_allowed < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , cmp : BinOpKind , left : & Expr < 'tcx > , right : & Expr < 'tcx > ,) -> bool { if (is_assoc_fn_without_type_instance (cx , left) || is_assoc_fn_without_type_instance (cx , right)) && ! is_expr_used_with_type_annotation (cx , expr) { return false ; } cx . typeck_results () . expr_ty (left) . peel_refs () . is_integral () && cx . typeck_results () . expr_ty (right) . peel_refs () . is_integral () && ! (cmp == BinOpKind :: Shl && is_zero_integer_const (cx , right , expr . span . ctxt ()) && integer_const (cx , left , expr . span . ctxt ()) == Some (1)) }
};
}
