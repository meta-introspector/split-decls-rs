// Generated macro for impl_7283 (impl)
macro_rules! Depcrate_missing_assert_messageimpl_7283 {
() => {
// Module: crate::missing_assert_message
// Provides: {"impl_7283"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MissingAssertMessage { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let Some (macro_call) = root_macro_call_first_node (cx , expr) else { return ; } ; let single_argument = match cx . tcx . get_diagnostic_name (macro_call . def_id) { Some (sym :: assert_macro | sym :: debug_assert_macro) => true , Some (sym :: assert_eq_macro | sym :: assert_ne_macro | sym :: debug_assert_eq_macro | sym :: debug_assert_ne_macro ,) => false , _ => return , } ; if is_in_test (cx . tcx , expr . hir_id) { return ; } let panic_expn = if single_argument { let Some ((_ , panic_expn)) = find_assert_args (cx , expr , macro_call . expn) else { return ; } ; panic_expn } else { let Some ((_ , _ , panic_expn)) = find_assert_eq_args (cx , expr , macro_call . expn) else { return ; } ; panic_expn } ; if let PanicExpn :: Empty = panic_expn { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , MISSING_ASSERT_MESSAGE , macro_call . span , "assert without any message" , | diag | { diag . help ("consider describing why the failing assert is problematic") ; } ,) ; } } }
};
}
