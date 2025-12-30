// Generated macro for AutoderefSnapshot (struct)
macro_rules! Depcrate_autoderefAutoderefSnapshot {
() => {
// Module: crate::autoderef
// Provides: {"AutoderefSnapshot"}
// Dependencies: {}
struct AutoderefSnapshot < 'tcx > { at_start : bool , reached_recursion_limit : bool , steps : Vec < (Ty < 'tcx > , AutoderefKind) > , cur_ty : Ty < 'tcx > , obligations : PredicateObligations < 'tcx > , }
};
}
