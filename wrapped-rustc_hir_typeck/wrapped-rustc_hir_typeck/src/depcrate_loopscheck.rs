// Generated macro for check (function)
macro_rules! Depcrate_loopscheck {
() => {
// Module: crate::loops
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , body : & 'tcx hir :: Body < 'tcx >) { let mut check = CheckLoopVisitor { tcx , cx_stack : vec ! [Normal] , block_breaks : Default :: default () } ; let cx = match tcx . def_kind (def_id) { DefKind :: AnonConst => AnonConst , _ => Fn , } ; check . with_context (cx , | v | v . visit_body (body)) ; check . report_outside_loop_error () ; }
};
}
