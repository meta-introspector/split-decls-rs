// Generated macro for emit_loan_kills (function)
macro_rules! Depcrate_polonius_legacy_loan_killsemit_loan_kills {
() => {
// Module: crate::polonius::legacy::loan_kills
// Provides: {"emit_loan_kills"}
// Dependencies: {}
# [doc = " Emit `loan_killed_at` and `cfg_edge` facts at the same time."] pub (super) fn emit_loan_kills < 'tcx > (tcx : TyCtxt < 'tcx > , facts : & mut PoloniusFacts , body : & Body < 'tcx > , location_table : & PoloniusLocationTable , borrow_set : & BorrowSet < 'tcx > ,) { let mut visitor = LoanKillsGenerator { borrow_set , tcx , location_table , facts , body } ; for (bb , data) in body . basic_blocks . iter_enumerated () { visitor . visit_basic_block_data (bb , data) ; } }
};
}
