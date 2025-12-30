// Generated macro for LiveVariablesVisitor (struct)
macro_rules! Depcrate_type_check_livenessLiveVariablesVisitor {
() => {
// Module: crate::type_check::liveness
// Provides: {"LiveVariablesVisitor"}
// Dependencies: {}
# [doc = " Visitor looking for regions that should be live within rvalues or calls."] struct LiveVariablesVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , liveness_constraints : & 'a mut LivenessValues , universal_regions : & 'a UniversalRegions < 'tcx > , polonius_liveness : & 'a mut Option < PoloniusLivenessContext > , }
};
}
