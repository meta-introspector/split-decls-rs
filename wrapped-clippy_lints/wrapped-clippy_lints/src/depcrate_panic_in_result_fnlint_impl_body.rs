// Generated macro for lint_impl_body (function)
macro_rules! Depcrate_panic_in_result_fnlint_impl_body {
() => {
// Module: crate::panic_in_result_fn
// Provides: {"lint_impl_body"}
// Dependencies: {}
fn lint_impl_body < 'tcx > (cx : & LateContext < 'tcx > , impl_span : Span , body : & 'tcx hir :: Body < 'tcx >) { let mut panics = Vec :: new () ; let _ : Option < ! > = for_each_expr (cx , body . value , | e | { let Some (macro_call) = root_macro_call_first_node (cx , e) else { return ControlFlow :: Continue (Descend :: Yes) ; } ; if ! is_inside_always_const_context (cx . tcx , e . hir_id) && (is_panic (cx , macro_call . def_id) || matches ! (cx . tcx . get_diagnostic_name (macro_call . def_id) , Some (sym :: assert_macro | sym :: assert_eq_macro | sym :: assert_ne_macro))) { panics . push (macro_call . span) ; ControlFlow :: Continue (Descend :: No) } else { ControlFlow :: Continue (Descend :: Yes) } }) ; if ! panics . is_empty () { span_lint_and_then (cx , PANIC_IN_RESULT_FN , impl_span , "used `panic!()` or assertion in a function that returns `Result`" , move | diag | { diag . help ("`panic!()` or assertions should not be used in a function that returns `Result` as `Result` is expected to return an error instead of crashing" ,) ; diag . span_note (panics , "return Err() instead of panicking") ; } ,) ; } }
};
}
