// Generated macro for LivenessResults (struct)
macro_rules! Depcrate_type_check_liveness_traceLivenessResults {
() => {
// Module: crate::type_check::liveness::trace
// Provides: {"LivenessResults"}
// Dependencies: {}
struct LivenessResults < 'a , 'typeck , 'tcx > { cx : LivenessContext < 'a , 'typeck , 'tcx > , # [doc = " Set of points that define the current local."] defs : DenseBitSet < PointIndex > , # [doc = " Points where the current variable is \"use live\" -- meaning"] # [doc = " that there is a future \"full use\" that may use its value."] use_live_at : IntervalSet < PointIndex > , # [doc = " Points where the current variable is \"drop live\" -- meaning"] # [doc = " that there is no future \"full use\" that may use its value, but"] # [doc = " there is a future drop."] drop_live_at : IntervalSet < PointIndex > , # [doc = " Locations where drops may occur."] drop_locations : Vec < Location > , # [doc = " Stack used when doing (reverse) DFS."] stack : Vec < PointIndex > , }
};
}
