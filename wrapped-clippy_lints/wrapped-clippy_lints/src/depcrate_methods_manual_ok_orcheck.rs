// Generated macro for check (function)
macro_rules! Depcrate_methods_manual_ok_orcheck {
() => {
// Module: crate::methods::manual_ok_or
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , recv : & 'tcx Expr < '_ > , or_expr : & 'tcx Expr < '_ > , map_expr : & 'tcx Expr < '_ > ,) { if let Some (method_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && let Some (impl_id) = cx . tcx . impl_of_assoc (method_id) && cx . tcx . type_of (impl_id) . instantiate_identity () . is_diag_item (cx , sym :: Option) && let ExprKind :: Call (err_path , [err_arg]) = or_expr . kind && err_path . res (cx) . ctor_parent (cx) . is_lang_item (cx , ResultErr) && is_ok_wrapping (cx , map_expr) && let Some (recv_snippet) = recv . span . get_source_text (cx) && let Some (err_arg_snippet) = err_arg . span . get_source_text (cx) && let Some (indent) = indent_of (cx , expr . span) { let reindented_err_arg_snippet = reindent_multiline (err_arg_snippet . as_str () , true , Some (indent + 4)) ; span_lint_and_sugg (cx , MANUAL_OK_OR , expr . span , "this pattern reimplements `Option::ok_or`" , "replace with" , format ! ("{recv_snippet}.ok_or({reindented_err_arg_snippet})") , Applicability :: MachineApplicable ,) ; } }
};
}
