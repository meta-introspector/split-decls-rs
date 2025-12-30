// Generated macro for AutoderefSnapshot (struct)
macro_rules! Depcrate_infer_autoderefAutoderefSnapshot {
() => {
// Module: crate::infer::autoderef
// Provides: {"AutoderefSnapshot"}
// Dependencies: {}
struct AutoderefSnapshot < 'db , Steps > { at_start : bool , reached_recursion_limit : bool , steps : Steps , cur_ty : Ty < 'db > , obligations : PredicateObligations < 'db > , }
};
}
