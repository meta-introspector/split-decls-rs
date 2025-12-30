// Generated macro for success (function)
macro_rules! Depcrate_coercionsuccess {
() => {
// Module: crate::coercion
// Provides: {"success"}
// Dependencies: {}
# [doc = " This always returns `Ok(...)`."] fn success < 'tcx > (adj : Vec < Adjustment < 'tcx > > , target : Ty < 'tcx > , obligations : PredicateObligations < 'tcx > ,) -> CoerceResult < 'tcx > { Ok (InferOk { value : (adj , target) , obligations }) }
};
}
