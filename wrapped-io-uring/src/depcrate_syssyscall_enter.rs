// Generated macro for SYSCALL_ENTER (const)
macro_rules! Depcrate_sysSYSCALL_ENTER {
() => {
// Module: crate::sys
// Provides: {"SYSCALL_ENTER"}
// Dependencies: {}
# [cfg (not (feature = "bindgen"))] const SYSCALL_ENTER : c_long = libc :: SYS_io_uring_enter ;
};
}
