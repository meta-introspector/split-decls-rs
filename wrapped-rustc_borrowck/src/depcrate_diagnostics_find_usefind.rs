// Generated macro for find (function)
macro_rules! Depcrate_diagnostics_find_usefind {
() => {
// Module: crate::diagnostics::find_use
// Provides: {"find"}
// Dependencies: {}
pub (crate) fn find < 'tcx > (body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , tcx : TyCtxt < 'tcx > , region_vid : RegionVid , start_point : Location ,) -> Option < Cause > { let mut uf = UseFinder { body , regioncx , tcx , region_vid , start_point } ; uf . find () }
};
}
