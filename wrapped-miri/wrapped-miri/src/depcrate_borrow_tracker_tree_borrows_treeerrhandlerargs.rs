// Generated macro for ErrHandlerArgs (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeErrHandlerArgs {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"ErrHandlerArgs"}
// Dependencies: {}
# [doc = " Data given to the error handler"] struct ErrHandlerArgs < 'node , InErr > { # [doc = " Kind of error that occurred"] error_kind : InErr , # [doc = " Tag that triggered the error (not the tag that was accessed,"] # [doc = " rather the parent tag that had insufficient permissions or the"] # [doc = " non-parent tag that had a protector)."] conflicting_info : & 'node NodeDebugInfo , # [doc = " Information about the tag that was accessed just before the"] # [doc = " error was triggered."] accessed_info : & 'node NodeDebugInfo , }
};
}
