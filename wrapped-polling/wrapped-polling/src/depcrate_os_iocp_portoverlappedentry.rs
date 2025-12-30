// Generated macro for OverlappedEntry (struct)
macro_rules! Depcrate_os_iocp_portOverlappedEntry {
() => {
// Module: crate::os::iocp::port
// Provides: {"OverlappedEntry"}
// Dependencies: {}
# [doc = " An `OVERLAPPED_ENTRY` resulting from an I/O completion port."] # [repr (transparent)] pub (super) struct OverlappedEntry < T : CompletionHandle > { # [doc = " The underlying entry."] entry : OVERLAPPED_ENTRY , # [doc = " We own the status block."] _marker : PhantomData < T > , }
};
}
