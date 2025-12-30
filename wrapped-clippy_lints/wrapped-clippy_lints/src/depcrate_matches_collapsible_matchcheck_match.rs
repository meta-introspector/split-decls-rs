// Generated macro for check_match (function)
macro_rules! Depcrate_matches_collapsible_matchcheck_match {
() => {
// Module: crate::matches::collapsible_match
// Provides: {"check_match"}
// Dependencies: {}
pub (super) fn check_match < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , arms : & 'tcx [Arm < '_ >] , msrv : Msrv) { if let Some (els_arm) = arms . iter () . rfind (| arm | arm_is_wild_like (cx , arm)) { for arm in arms { check_arm (cx , true , arm . pat , expr , arm . body , arm . guard , Some (els_arm . body) , msrv) ; } } }
};
}
