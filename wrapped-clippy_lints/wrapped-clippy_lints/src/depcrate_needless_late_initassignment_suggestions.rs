// Generated macro for assignment_suggestions (function)
macro_rules! Depcrate_needless_late_initassignment_suggestions {
() => {
// Module: crate::needless_late_init
// Provides: {"assignment_suggestions"}
// Dependencies: {}
fn assignment_suggestions < 'tcx > (cx : & LateContext < 'tcx > , binding_id : HirId , exprs : impl IntoIterator < Item = & 'tcx Expr < 'tcx > > ,) -> Option < (Applicability , Vec < (Span , String) >) > { let mut assignments = Vec :: new () ; for expr in exprs { let ty = cx . typeck_results () . expr_ty (expr) ; if ty . is_never () { continue ; } if ! ty . is_unit () { return None ; } let assign = LocalAssign :: new (cx , expr , binding_id) ? ; assignments . push (assign) ; } let suggestions = assignments . iter () . flat_map (| assignment | { let mut spans = vec ! [assignment . span . until (assignment . rhs_span)] ; if assignment . rhs_span . hi () != assignment . span . hi () { spans . push (assignment . rhs_span . shrink_to_hi () . with_hi (assignment . span . hi ())) ; } spans }) . map (| span | (span , String :: new ())) . collect :: < Vec < (Span , String) > > () ; match suggestions . len () { 0 => None , 1 => Some ((Applicability :: MachineApplicable , suggestions)) , _ => Some ((Applicability :: Unspecified , suggestions)) , } }
};
}
