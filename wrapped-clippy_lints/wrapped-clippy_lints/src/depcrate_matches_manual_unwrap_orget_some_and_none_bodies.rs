// Generated macro for get_some_and_none_bodies (function)
macro_rules! Depcrate_matches_manual_unwrap_orget_some_and_none_bodies {
() => {
// Module: crate::matches::manual_unwrap_or
// Provides: {"get_some_and_none_bodies"}
// Dependencies: {}
fn get_some_and_none_bodies < 'tcx > (cx : & LateContext < 'tcx > , arm1 : & 'tcx Arm < 'tcx > , arm2 : & 'tcx Arm < 'tcx > ,) -> Option < ((& 'tcx Expr < 'tcx > , HirId) , & 'tcx Expr < 'tcx >) > { if let Some (binding_id) = get_some (cx , arm1 . pat) && let Some (body_none) = get_none (cx , arm2 , true) { Some (((arm1 . body , binding_id) , body_none)) } else if let Some (body_none) = get_none (cx , arm1 , false) && let Some (binding_id) = get_some (cx , arm2 . pat) { Some (((arm2 . body , binding_id) , body_none)) } else { None } }
};
}
