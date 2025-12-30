// Generated macro for check_match (function)
macro_rules! Depcrate_matches_manual_mapcheck_match {
() => {
// Module: crate::matches::manual_map
// Provides: {"check_match"}
// Dependencies: {}
pub (super) fn check_match < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , scrutinee : & 'tcx Expr < '_ > , arms : & 'tcx [Arm < '_ >] ,) { if let [arm1 , arm2] = arms && arm1 . guard . is_none () && arm2 . guard . is_none () { check (cx , expr , scrutinee , arm1 . pat , arm1 . body , Some (arm2 . pat) , arm2 . body) ; } }
};
}
