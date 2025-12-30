// Generated macro for calculate_borrows_out_of_scope_at_location (function)
macro_rules! Depcrate_dataflowcalculate_borrows_out_of_scope_at_location {
() => {
// Module: crate::dataflow
// Provides: {"calculate_borrows_out_of_scope_at_location"}
// Dependencies: {}
pub fn calculate_borrows_out_of_scope_at_location < 'tcx > (body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , borrow_set : & BorrowSet < 'tcx > ,) -> FxIndexMap < Location , Vec < BorrowIndex > > { OutOfScopePrecomputer :: compute (body , regioncx , borrow_set) }
};
}
