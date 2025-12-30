// Generated macro for find_type_name (function)
macro_rules! Depcrate_matches_manual_unwrap_orfind_type_name {
() => {
// Module: crate::matches::manual_unwrap_or
// Provides: {"find_type_name"}
// Dependencies: {}
fn find_type_name < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> Option < & 'static str > { match ty . opt_diag_name (cx) ? { sym :: Option => Some ("Option") , sym :: Result => Some ("Result") , _ => None , } }
};
}
