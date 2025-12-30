// Generated macro for SccUniverse (struct)
macro_rules! Depcrate_infer_region_constraints_leak_checkSccUniverse {
() => {
// Module: crate::infer::region_constraints::leak_check
// Provides: {"SccUniverse"}
// Dependencies: {}
# [doc = " Tracks the \"minimum universe\" for each SCC, along with some region that"] # [doc = " caused it to change."] # [derive (Copy , Clone , Debug)] struct SccUniverse < 'tcx > { # [doc = " For some SCC S, the minimum universe of:"] # [doc = ""] # [doc = " * each region R in S"] # [doc = " * each SCC S1 such that S: S1"] universe : ty :: UniverseIndex , # [doc = " Some region that caused `universe` to be what it is."] region : Option < ty :: Region < 'tcx > > , }
};
}
