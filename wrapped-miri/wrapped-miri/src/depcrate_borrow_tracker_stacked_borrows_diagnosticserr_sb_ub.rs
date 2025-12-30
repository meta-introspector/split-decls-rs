// Generated macro for err_sb_ub (function)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticserr_sb_ub {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"err_sb_ub"}
// Dependencies: {}
# [doc = " Error reporting"] fn err_sb_ub < 'tcx > (msg : String , help : Vec < String > , history : Option < TagHistory > ,) -> InterpErrorKind < 'tcx > { err_machine_stop ! (TerminationInfo :: StackedBorrowsUb { msg , help , history }) }
};
}
