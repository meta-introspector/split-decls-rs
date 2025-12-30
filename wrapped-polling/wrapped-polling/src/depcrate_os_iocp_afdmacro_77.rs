// Generated macro for macro_77 (macro)
macro_rules! Depcrate_os_iocp_afdmacro_77 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"macro_77"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [doc = " An I/O status block paired with some auxiliary data."] # [repr (C)] pub (super) struct IoStatusBlock < T > { iosb : UnsafeCell < IO_STATUS_BLOCK >, in_use : AtomicBool , # [pin] data : T , # [pin] _marker : PhantomPinned , } }
};
}
