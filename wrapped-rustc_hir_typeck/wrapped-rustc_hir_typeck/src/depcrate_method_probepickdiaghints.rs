// Generated macro for PickDiagHints (struct)
macro_rules! Depcrate_method_probePickDiagHints {
() => {
// Module: crate::method::probe
// Provides: {"PickDiagHints"}
// Dependencies: {}
# [doc = " Extra information required only for error reporting."] # [derive (Debug)] struct PickDiagHints < 'a , 'tcx > { # [doc = " Unstable candidates alongside the stable ones."] unstable_candidates : Option < Vec < (Candidate < 'tcx > , Symbol) > > , # [doc = " Collects near misses when trait bounds for type parameters are unsatisfied and is only used"] # [doc = " for error reporting"] unsatisfied_predicates : & 'a mut Vec < (ty :: Predicate < 'tcx > , Option < ty :: Predicate < 'tcx > > , Option < ObligationCause < 'tcx > > ,) > , }
};
}
