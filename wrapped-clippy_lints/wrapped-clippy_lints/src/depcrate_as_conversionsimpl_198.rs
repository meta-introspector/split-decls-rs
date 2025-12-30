// Generated macro for impl_198 (impl)
macro_rules! Depcrate_as_conversionsimpl_198 {
() => {
// Module: crate::as_conversions
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for AsConversions { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if let ExprKind :: Cast (_ , _) = expr . kind && ! expr . span . in_external_macro (cx . sess () . source_map ()) && ! is_from_proc_macro (cx , expr) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , AS_CONVERSIONS , expr . span , "using a potentially dangerous silent `as` conversion" , | diag | { diag . help ("consider using a safe wrapper for this conversion") ; } ,) ; } } }
};
}
