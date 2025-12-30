// Generated macro for check (function)
macro_rules! Depcrate_methods_unwrap_expect_usedcheck {
() => {
// Module: crate::methods::unwrap_expect_used
// Provides: {"check"}
// Dependencies: {}
# [doc = " Lint usage of `unwrap` or `unwrap_err` for `Result` and `unwrap()` for `Option` (and their"] # [doc = " `expect` counterparts)."] pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , is_err : bool , allow_unwrap_in_consts : bool , allow_unwrap_in_tests : bool , variant : Variant ,) { let ty = cx . typeck_results () . expr_ty (recv) . peel_refs () ; let (kind , none_value , none_prefix) = if ty . is_diag_item (cx , sym :: Option) && ! is_err { ("an `Option`" , "None" , "") } else if ty . is_diag_item (cx , sym :: Result) && let ty :: Adt (_ , substs) = ty . kind () && let Some (t_or_e_ty) = substs [usize :: from (! is_err)] . as_type () { if is_never_like (t_or_e_ty) { return ; } ("a `Result`" , if is_err { "Ok" } else { "Err" } , "an ") } else { return ; } ; let method_suffix = if is_err { "_err" } else { "" } ; if allow_unwrap_in_tests && is_in_test (cx . tcx , expr . hir_id) { return ; } if allow_unwrap_in_consts && is_inside_always_const_context (cx . tcx , expr . hir_id) { return ; } span_lint_and_then (cx , variant . lint () , expr . span , format ! ("used `{}()` on {kind} value" , variant . method_name (is_err)) , | diag | { diag . note (format ! ("if this value is {none_prefix}`{none_value}`, it will panic")) ; if variant == Variant :: Unwrap && is_lint_allowed (cx , EXPECT_USED , expr . hir_id) { diag . help (format ! ("consider using `expect{method_suffix}()` to provide a better panic message")) ; } } ,) ; }
};
}
