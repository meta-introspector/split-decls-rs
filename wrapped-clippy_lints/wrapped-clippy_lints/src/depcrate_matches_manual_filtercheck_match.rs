// Generated macro for check_match (function)
macro_rules! Depcrate_matches_manual_filtercheck_match {
() => {
// Module: crate::matches::manual_filter
// Provides: {"check_match"}
// Dependencies: {}
pub (super) fn check_match < 'tcx > (cx : & LateContext < 'tcx > , scrutinee : & 'tcx Expr < '_ > , arms : & 'tcx [Arm < '_ >] , expr : & 'tcx Expr < '_ > ,) { let ty = cx . typeck_results () . expr_ty (expr) ; if ty . is_diag_item (cx , sym :: Option) && let [first_arm , second_arm] = arms && first_arm . guard . is_none () && second_arm . guard . is_none () { check (cx , expr , scrutinee , first_arm . pat , first_arm . body , Some (second_arm . pat) , second_arm . body ,) ; } }
};
}
