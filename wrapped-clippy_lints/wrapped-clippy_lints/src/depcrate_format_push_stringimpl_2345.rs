// Generated macro for impl_2345 (impl)
macro_rules! Depcrate_format_push_stringimpl_2345 {
() => {
// Module: crate::format_push_string
// Provides: {"impl_2345"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for FormatPushString { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let arg = match expr . kind { ExprKind :: MethodCall (_ , _ , [arg] , _) => { if let Some (fn_def_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && cx . tcx . is_diagnostic_item (sym :: string_push_str , fn_def_id) { arg } else { return ; } } , ExprKind :: AssignOp (op , left , arg) if op . node == AssignOpKind :: AddAssign && is_string (cx , left) => arg , _ => return , } ; if is_format (cx , arg) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , FORMAT_PUSH_STRING , expr . span , "`format!(..)` appended to existing `String`" , | diag | { diag . help ("consider using `write!` to avoid the extra allocation") ; } ,) ; } } }
};
}
