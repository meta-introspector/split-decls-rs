// Generated macro for DisplayFmtWrapper (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsDisplayFmtWrapper {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"DisplayFmtWrapper"}
// Dependencies: {}
# [doc = " Pretty-printing details"] # [doc = ""] # [doc = " Example:"] # [doc = " ```rust,ignore (private type)"] # [doc = " DisplayFmtWrapper {"] # [doc = "     top: '>',"] # [doc = "     bot: '<',"] # [doc = "     warning_text: \"Some tags have been hidden\","] # [doc = " }"] # [doc = " ```"] # [doc = " will wrap the entire text with"] # [doc = " ```text"] # [doc = " >>>>>>>>>>>>>>>>>>>>>>>>>>"] # [doc = " Some tags have been hidden"] # [doc = ""] # [doc = " [ main display here ]"] # [doc = ""] # [doc = " <<<<<<<<<<<<<<<<<<<<<<<<<<"] # [doc = " ```"] struct DisplayFmtWrapper { # [doc = " Character repeated to make the upper border."] top : char , # [doc = " Character repeated to make the lower border."] bot : char , # [doc = " Warning about some tags (unnamed) being hidden."] warning_text : S , }
};
}
