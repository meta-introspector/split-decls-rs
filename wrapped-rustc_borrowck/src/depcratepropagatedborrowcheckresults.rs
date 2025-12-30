// Generated macro for PropagatedBorrowCheckResults (struct)
macro_rules! DepcratePropagatedBorrowCheckResults {
() => {
// Module: crate
// Provides: {"PropagatedBorrowCheckResults"}
// Dependencies: {}
# [doc = " Data propagated to the typeck parent by nested items."] # [doc = " This should always be empty for the typeck root."] # [derive (Debug)] struct PropagatedBorrowCheckResults < 'tcx > { closure_requirements : Option < ClosureRegionRequirements < 'tcx > > , used_mut_upvars : SmallVec < [FieldIdx ; 8] > , }
};
}
