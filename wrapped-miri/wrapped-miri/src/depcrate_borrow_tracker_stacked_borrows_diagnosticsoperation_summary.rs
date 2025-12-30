// Generated macro for operation_summary (function)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticsoperation_summary {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"operation_summary"}
// Dependencies: {}
fn operation_summary (operation : & str , alloc_id : AllocId , alloc_range : AllocRange) -> String { format ! ("this error occurs as part of {operation} at {alloc_id:?}{alloc_range:?}") }
};
}
