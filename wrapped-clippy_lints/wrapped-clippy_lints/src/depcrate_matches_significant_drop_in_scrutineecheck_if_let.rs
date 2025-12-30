// Generated macro for check_if_let (function)
macro_rules! Depcrate_matches_significant_drop_in_scrutineecheck_if_let {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"check_if_let"}
// Dependencies: {}
pub (super) fn check_if_let < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , scrutinee : & 'tcx Expr < '_ > , if_then : & 'tcx Expr < '_ > , if_else : Option < & 'tcx Expr < '_ > > ,) { if is_lint_allowed (cx , SIGNIFICANT_DROP_IN_SCRUTINEE , expr . hir_id) { return ; } let message = "temporary with significant `Drop` in `if let` scrutinee will live until the end of the `if let` expression" ; if let Some (if_else) = if_else { check (cx , expr , scrutinee , & [if_then , if_else] , message , Suggestion :: Emit) ; } else { check (cx , expr , scrutinee , & [if_then] , message , Suggestion :: Emit) ; } }
};
}
