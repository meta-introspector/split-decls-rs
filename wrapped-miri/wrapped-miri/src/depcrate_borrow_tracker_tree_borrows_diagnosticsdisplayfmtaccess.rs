// Generated macro for DisplayFmtAccess (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsDisplayFmtAccess {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"DisplayFmtAccess"}
// Dependencies: {}
# [doc = " How to show whether a location has been accessed"] # [doc = ""] # [doc = " Example:"] # [doc = " ```rust,ignore (private type)"] # [doc = " DisplayFmtAccess {"] # [doc = "     yes: \" \","] # [doc = "     no: \"?\","] # [doc = "     meh: \"_\","] # [doc = " }"] # [doc = " ```"] # [doc = " will show states as"] # [doc = " ```text"] # [doc = "  Act"] # [doc = " ?Res"] # [doc = " ____"] # [doc = " ```"] struct DisplayFmtAccess { # [doc = " Used when `State.initialized = true`."] yes : S , # [doc = " Used when `State.initialized = false`."] # [doc = " Should have the same width as `yes`."] no : S , # [doc = " Used when there is no `State`."] # [doc = " Should have the same width as `yes`."] meh : S , }
};
}
