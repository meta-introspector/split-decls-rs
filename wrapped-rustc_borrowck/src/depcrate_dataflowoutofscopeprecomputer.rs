// Generated macro for OutOfScopePrecomputer (struct)
macro_rules! Depcrate_dataflowOutOfScopePrecomputer {
() => {
// Module: crate::dataflow
// Provides: {"OutOfScopePrecomputer"}
// Dependencies: {}
struct OutOfScopePrecomputer < 'a , 'tcx > { visited : DenseBitSet < mir :: BasicBlock > , visit_stack : Vec < mir :: BasicBlock > , body : & 'a Body < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , borrows_out_of_scope_at_location : FxIndexMap < Location , Vec < BorrowIndex > > , }
};
}
