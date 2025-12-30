// Generated macro for VSOCK_MAP (static)
macro_rules! Depcrate_executor_vsockVSOCK_MAP {
() => {
// Module: crate::executor::vsock
// Provides: {"VSOCK_MAP"}
// Dependencies: {}
pub (crate) static VSOCK_MAP : InterruptTicketMutex < VsockMap > = InterruptTicketMutex :: new (VsockMap :: new ()) ;
};
}
