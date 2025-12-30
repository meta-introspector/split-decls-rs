// Generated macro for IoCompletionPort (struct)
macro_rules! Depcrate_os_iocp_portIoCompletionPort {
() => {
// Module: crate::os::iocp::port
// Provides: {"IoCompletionPort"}
// Dependencies: {}
# [doc = " A handle to the I/O completion port."] pub (super) struct IoCompletionPort < T > { # [doc = " The underlying handle."] handle : HANDLE , # [doc = " We own the status block."] _marker : PhantomData < T > , }
};
}
