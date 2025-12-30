// Generated macro for sys_umask (function)
macro_rules! Depcrate_syscallssys_umask {
() => {
// Module: crate::syscalls
// Provides: {"sys_umask"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_umask (umask : u32) -> u32 { crate :: fs :: umask (AccessPermission :: from_bits_truncate (umask)) . bits () }
};
}
