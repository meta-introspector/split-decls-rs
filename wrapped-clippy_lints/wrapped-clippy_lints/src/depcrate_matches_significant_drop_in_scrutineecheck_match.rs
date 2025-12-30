// Generated macro for check_match (function)
macro_rules! Depcrate_matches_significant_drop_in_scrutineecheck_match {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"check_match"}
// Dependencies: {}
pub (super) fn check_match < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , scrutinee : & 'tcx Expr < '_ > , arms : & 'tcx [Arm < '_ >] , source : MatchSource ,) { if is_lint_allowed (cx , SIGNIFICANT_DROP_IN_SCRUTINEE , expr . hir_id) { return ; } let scrutinee = match (source , & scrutinee . kind) { (MatchSource :: ForLoopDesugar , ExprKind :: Call (_ , [e])) => e , _ => scrutinee , } ; let message = if source == MatchSource :: Normal { "temporary with significant `Drop` in `match` scrutinee will live until the end of the `match` expression" } else { "temporary with significant `Drop` in `for` loop condition will live until the end of the `for` expression" } ; let arms = arms . iter () . map (| arm | arm . body) . collect :: < Vec < _ > > () ; check (cx , expr , scrutinee , & arms , message , Suggestion :: Emit) ; }
};
}
