// Generated macro for no_valid_exposed_references_error (function)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsno_valid_exposed_references_error {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"no_valid_exposed_references_error"}
// Dependencies: {}
# [doc = " Cannot access this allocation with wildcard provenance, as there are no"] # [doc = " valid exposed references for this access kind."] pub fn no_valid_exposed_references_error < 'tcx > (alloc_id : AllocId , offset : u64 , access_cause : AccessCause ,) -> InterpErrorKind < 'tcx > { let title = format ! ("{access_cause} through <wildcard> at {alloc_id:?}[{offset:#x}] is forbidden") ; let details = vec ! [format ! ("there are no exposed tags which may perform this access here")] ; let history = HistoryData :: default () ; err_machine_stop ! (TerminationInfo :: TreeBorrowsUb { title , details , history }) }
};
}
