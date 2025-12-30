// Generated macro for HistoryData (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsHistoryData {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"HistoryData"}
// Dependencies: {}
# [doc = " History formatted for use by `src/diagnostics.rs`."] # [doc = ""] # [doc = " NOTE: needs to be `Send` because of a bound on `MachineStopType`, hence"] # [doc = " the use of `SpanData` rather than `Span`."] # [derive (Debug , Clone , Default)] pub struct HistoryData { pub events : Vec < (Option < SpanData > , String) > , }
};
}
