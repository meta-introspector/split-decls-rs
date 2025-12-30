// Generated macro for len_arg (function)
macro_rules! Depcrate_manual_striplen_arg {
() => {
// Module: crate::manual_strip
// Provides: {"len_arg"}
// Dependencies: {}
fn len_arg < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < & 'tcx Expr < 'tcx > > { if let ExprKind :: MethodCall (_ , arg , [] , _) = expr . kind && let Some (method_def_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && cx . tcx . is_diagnostic_item (sym :: str_len , method_def_id) { Some (arg) } else { None } }
};
}
