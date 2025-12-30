// Generated macro for TransitionError (enum)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsTransitionError {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"TransitionError"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] pub (super) enum TransitionError { # [doc = " This access is not allowed because some parent tag has insufficient permissions."] # [doc = " For example, if a tag is `Frozen` and encounters a child write this will"] # [doc = " produce a `ChildAccessForbidden(Frozen)`."] # [doc = " This kind of error can only occur on child accesses."] ChildAccessForbidden (Permission) , # [doc = " A protector was triggered due to an invalid transition that loses"] # [doc = " too much permissions."] # [doc = " For example, if a protected tag goes from `Active` to `Disabled` due"] # [doc = " to a foreign write this will produce a `ProtectedDisabled(Active)`."] # [doc = " This kind of error can only occur on foreign accesses."] ProtectedDisabled (Permission) , # [doc = " Cannot deallocate because some tag in the allocation is strongly protected."] # [doc = " This kind of error can only occur on deallocations."] ProtectedDealloc , }
};
}
