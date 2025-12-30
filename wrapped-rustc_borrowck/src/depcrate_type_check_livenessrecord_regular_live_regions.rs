// Generated macro for record_regular_live_regions (function)
macro_rules! Depcrate_type_check_livenessrecord_regular_live_regions {
() => {
// Module: crate::type_check::liveness
// Provides: {"record_regular_live_regions"}
// Dependencies: {}
# [doc = " Some variables are \"regular live\" at `location` -- i.e., they may be used later. This means that"] # [doc = " all regions appearing in their type must be live at `location`."] fn record_regular_live_regions < 'tcx > (tcx : TyCtxt < 'tcx > , liveness_constraints : & mut LivenessValues , universal_regions : & UniversalRegions < 'tcx > , polonius_liveness : & mut Option < PoloniusLivenessContext > , body : & Body < 'tcx > ,) { let mut visitor = LiveVariablesVisitor { tcx , liveness_constraints , universal_regions , polonius_liveness } ; for (bb , data) in body . basic_blocks . iter_enumerated () { visitor . visit_basic_block_data (bb , data) ; } }
};
}
