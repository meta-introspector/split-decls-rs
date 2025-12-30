// Generated macro for CompletionHandle (trait)
macro_rules! Depcrate_os_iocp_portCompletionHandle {
() => {
// Module: crate::os::iocp::port
// Provides: {"CompletionHandle"}
// Dependencies: {}
# [doc = " The pointer to a completion block."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This must be a valid completion block."] pub (super) unsafe trait CompletionHandle : Deref + Sized { # [doc = " Type of the completion block."] type Completion : Completion ; # [doc = " Get a pointer to the completion block."] # [doc = ""] # [doc = " The pointer is pinned since the underlying object should not be moved"] # [doc = " after creation. This prevents it from being invalidated while it's"] # [doc = " used in an overlapped operation."] fn get (& self) -> Pin < & Self :: Completion > ; # [doc = " Convert this block into a pointer that can be passed as `*mut OVERLAPPED`."] fn into_ptr (this : Self) -> * mut OVERLAPPED ; # [doc = " Convert a pointer that was passed as `*mut OVERLAPPED` into a pointer to this block."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This must be a valid pointer to a completion block."] unsafe fn from_ptr (ptr : * mut OVERLAPPED) -> Self ; # [doc = " Convert to a pointer without losing ownership."] fn as_ptr (& self) -> * mut OVERLAPPED ; }
};
}
