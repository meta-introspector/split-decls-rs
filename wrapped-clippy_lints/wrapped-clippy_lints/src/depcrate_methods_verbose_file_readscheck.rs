// Generated macro for check (function)
macro_rules! Depcrate_methods_verbose_file_readscheck {
() => {
// Module: crate::methods::verbose_file_reads
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , recv : & 'tcx Expr < '_ > , (msg , help) : (& 'static str , & 'static str) ,) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: IoRead) && matches ! (recv . kind , ExprKind :: Path (QPath :: Resolved (None , _))) && cx . typeck_results () . expr_ty_adjusted (recv) . peel_refs () . is_diag_item (cx , sym :: File) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , VERBOSE_FILE_READS , expr . span , msg , | diag | { diag . help (help) ; }) ; } }
};
}
