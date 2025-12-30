// Generated macro for check_while_let (function)
macro_rules! Depcrate_matches_significant_drop_in_scrutineecheck_while_let {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"check_while_let"}
// Dependencies: {}
pub (super) fn check_while_let < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , scrutinee : & 'tcx Expr < '_ > , body : & 'tcx Expr < '_ > ,) { if is_lint_allowed (cx , SIGNIFICANT_DROP_IN_SCRUTINEE , expr . hir_id) { return ; } check (cx , expr , scrutinee , & [body] , "temporary with significant `Drop` in `while let` scrutinee will live until the end of the `while let` expression" , Suggestion :: DontEmit ,) ; }
};
}
