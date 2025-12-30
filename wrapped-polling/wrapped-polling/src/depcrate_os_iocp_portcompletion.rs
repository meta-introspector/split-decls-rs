// Generated macro for Completion (trait)
macro_rules! Depcrate_os_iocp_portCompletion {
() => {
// Module: crate::os::iocp::port
// Provides: {"Completion"}
// Dependencies: {}
# [doc = " A completion block which can be used with I/O completion ports."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This must be a valid completion block."] pub (super) unsafe trait Completion { # [doc = " Signal to the completion block that we are about to start an operation."] fn try_lock (self : Pin < & Self >) -> bool ; # [doc = " Unlock the completion block."] unsafe fn unlock (self : Pin < & Self >) ; }
};
}
