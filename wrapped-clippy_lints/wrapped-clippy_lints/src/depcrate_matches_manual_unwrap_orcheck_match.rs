// Generated macro for check_match (function)
macro_rules! Depcrate_matches_manual_unwrap_orcheck_match {
() => {
// Module: crate::matches::manual_unwrap_or
// Provides: {"check_match"}
// Dependencies: {}
pub fn check_match < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , scrutinee : & 'tcx Expr < 'tcx > , arms : & 'tcx [Arm < 'tcx >] ,) { if let [arm1 , arm2] = arms && arm1 . guard . is_none () && arm2 . guard . is_none () && let Some (((body_some , binding_id) , body_none)) = get_some_and_none_bodies (cx , arm1 , arm2) { handle (cx , expr , "match" , scrutinee , body_some , body_none , binding_id) ; } }
};
}
