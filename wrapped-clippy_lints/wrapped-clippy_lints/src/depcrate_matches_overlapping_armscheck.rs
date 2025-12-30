// Generated macro for check (function)
macro_rules! Depcrate_matches_overlapping_armscheck {
() => {
// Module: crate::matches::overlapping_arms
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check < 'tcx > (cx : & LateContext < 'tcx > , ex : & 'tcx Expr < '_ > , arms : & 'tcx [Arm < '_ >]) { if arms . len () >= 2 && cx . typeck_results () . expr_ty (ex) . is_integral () { let ranges = all_ranges (cx , arms , cx . typeck_results () . expr_ty (ex)) ; if ! ranges . is_empty () && let Some ((start , end)) = overlapping (& ranges) { span_lint_and_note (cx , MATCH_OVERLAPPING_ARM , start . span , "some ranges overlap" , Some (end . span) , "overlaps with this" ,) ; } } }
};
}
