// Generated macro for impl_553 (impl)
macro_rules! Depcrate_bool_to_int_with_ifimpl_553 {
() => {
// Module: crate::bool_to_int_with_if
// Provides: {"impl_553"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for BoolToIntWithIf { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if ! expr . span . from_expansion () && let Some (higher :: If { cond , then , r#else : Some (r#else) , }) = higher :: If :: hir (expr) && let Some (then_lit) = as_int_bool_lit (then) && let Some (else_lit) = as_int_bool_lit (r#else) && then_lit != else_lit && ! is_in_const_context (cx) { let ty = cx . typeck_results () . expr_ty (then) ; let mut applicability = if span_contains_comment (cx . sess () . source_map () , expr . span) { Applicability :: MaybeIncorrect } else { Applicability :: MachineApplicable } ; let snippet = { let mut sugg = Sugg :: hir_with_context (cx , cond , expr . span . ctxt () , ".." , & mut applicability) ; if ! then_lit { sugg = ! sugg ; } sugg } ; let suggestion = { let mut s = Sugg :: NonParen (format ! ("{ty}::from({snippet})") . into ()) ; if is_else_clause (cx . tcx , expr) { s = s . blockify () ; } s } ; let into_snippet = snippet . clone () . maybe_paren () ; let as_snippet = snippet . as_ty (ty) ; span_lint_and_then (cx , BOOL_TO_INT_WITH_IF , expr . span , "boolean to int conversion using if" , | diag | { diag . span_suggestion (expr . span , "replace with from" , suggestion , applicability) ; diag . note (format ! ("`{as_snippet}` or `{into_snippet}.into()` can also be valid options")) ; } ,) ; } } }
};
}
