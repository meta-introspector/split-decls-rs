// Generated macro for check_cast_method (function)
macro_rules! Depcrate_casts_cast_ptr_alignmentcheck_cast_method {
() => {
// Module: crate::casts::cast_ptr_alignment
// Provides: {"check_cast_method"}
// Dependencies: {}
pub (super) fn check_cast_method (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: MethodCall (method_path , self_arg , [] , _) = & expr . kind && method_path . ident . name == sym :: cast && let Some (generic_args) = method_path . args && let [GenericArg :: Type (cast_to)] = generic_args . args && ! is_hir_ty_cfg_dependant (cx , cast_to . as_unambig_ty ()) { let (cast_from , cast_to) = (cx . typeck_results () . expr_ty (self_arg) , cx . typeck_results () . expr_ty (expr)) ; lint_cast_ptr_alignment (cx , expr , cast_from , cast_to) ; } }
};
}
