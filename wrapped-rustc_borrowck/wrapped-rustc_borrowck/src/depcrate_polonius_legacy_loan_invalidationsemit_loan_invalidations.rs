// Generated macro for emit_loan_invalidations (function)
macro_rules! Depcrate_polonius_legacy_loan_invalidationsemit_loan_invalidations {
() => {
// Module: crate::polonius::legacy::loan_invalidations
// Provides: {"emit_loan_invalidations"}
// Dependencies: {}
# [doc = " Emit `loan_invalidated_at` facts."] pub (super) fn emit_loan_invalidations < 'tcx > (tcx : TyCtxt < 'tcx > , facts : & mut PoloniusFacts , body : & Body < 'tcx > , location_table : & PoloniusLocationTable , borrow_set : & BorrowSet < 'tcx > ,) { let dominators = body . basic_blocks . dominators () ; let mut visitor = LoanInvalidationsGenerator { facts , borrow_set , tcx , location_table , body , dominators } ; visitor . visit_body (body) ; }
};
}
