// Generated macro for TbError (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsTbError {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"TbError"}
// Dependencies: {}
# [doc = " Failures that can occur during the execution of Tree Borrows procedures."] pub (super) struct TbError < 'node > { # [doc = " What failure occurred."] pub error_kind : TransitionError , # [doc = " The allocation in which the error is happening."] pub alloc_id : AllocId , # [doc = " The offset (into the allocation) at which the conflict occurred."] pub error_offset : u64 , # [doc = " The tag on which the error was triggered."] # [doc = " On protector violations, this is the tag that was protected."] # [doc = " On accesses rejected due to insufficient permissions, this is the"] # [doc = " tag that lacked those permissions."] pub conflicting_info : & 'node NodeDebugInfo , pub access_cause : AccessCause , # [doc = " Which tag the access that caused this error was made through, i.e."] # [doc = " which tag was used to read/write/deallocate."] pub accessed_info : & 'node NodeDebugInfo , }
};
}
