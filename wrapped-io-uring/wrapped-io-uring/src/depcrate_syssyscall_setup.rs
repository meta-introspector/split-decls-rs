// Generated macro for SYSCALL_SETUP (const)
macro_rules! Depcrate_sysSYSCALL_SETUP {
() => {
// Module: crate::sys
// Provides: {"SYSCALL_SETUP"}
// Dependencies: {}
# [cfg (not (feature = "bindgen"))] const SYSCALL_SETUP : c_long = libc :: SYS_io_uring_setup ;
};
}
