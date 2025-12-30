// Generated macro for History (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsHistory {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"History"}
// Dependencies: {}
# [doc = " List of all events that affected a tag."] # [doc = " NOTE: not all of these events are relevant for a particular location,"] # [doc = " the events should be filtered before the generation of diagnostics."] # [doc = " Available filtering methods include `History::forget` and `History::extract_relevant`."] # [derive (Clone , Debug)] pub struct History { tag : BorTag , created : (Span , Permission) , events : Vec < Event > , }
};
}
