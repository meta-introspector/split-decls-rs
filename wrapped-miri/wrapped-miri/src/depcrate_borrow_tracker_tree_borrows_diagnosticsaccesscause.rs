// Generated macro for AccessCause (enum)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsAccessCause {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"AccessCause"}
// Dependencies: {}
# [doc = " Cause of an access: either a real access or one"] # [doc = " inserted by Tree Borrows due to a reborrow or a deallocation."] # [derive (Clone , Copy , Debug)] pub enum AccessCause { Explicit (AccessKind) , Reborrow , Dealloc , FnExit (AccessKind) , }
};
}
