// Generated macro for check (function)
macro_rules! Depcrate_methods_unbuffered_bytescheck {
() => {
// Module: crate::methods::unbuffered_bytes
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , recv : & hir :: Expr < '_ >) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: IoRead) && let Some (buf_read) = cx . tcx . get_diagnostic_item (sym :: IoBufRead) && let ty = cx . typeck_results () . expr_ty_adjusted (recv) && ! implements_trait (cx , ty , buf_read , & []) { span_lint_and_help (cx , UNBUFFERED_BYTES , expr . span , "calling .bytes() is very inefficient when data is not in memory" , None , "consider using `BufReader`" ,) ; } }
};
}
