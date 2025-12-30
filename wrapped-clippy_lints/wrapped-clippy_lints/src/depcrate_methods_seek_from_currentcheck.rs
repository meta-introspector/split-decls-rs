// Generated macro for check (function)
macro_rules! Depcrate_methods_seek_from_currentcheck {
() => {
// Module: crate::methods::seek_from_current
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , recv : & 'tcx Expr < '_ > , arg : & 'tcx Expr < '_ >) { let ty = cx . typeck_results () . expr_ty (recv) ; if let Some (def_id) = cx . tcx . get_diagnostic_item (sym :: IoSeek) && implements_trait (cx , ty , def_id , & []) && arg_is_seek_from_current (cx , arg) { let mut applicability = Applicability :: MachineApplicable ; let snip = snippet_with_applicability (cx , recv . span , ".." , & mut applicability) ; span_lint_and_sugg (cx , SEEK_FROM_CURRENT , expr . span , "using `SeekFrom::Current` to start from current position" , "replace with" , format ! ("{snip}.stream_position()") , applicability ,) ; } }
};
}
