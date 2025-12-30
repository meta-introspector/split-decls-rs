// Generated macro for sys_getpagesize (function)
macro_rules! Depcrate_syscalls_systemsys_getpagesize {
() => {
// Module: crate::syscalls::system
// Provides: {"sys_getpagesize"}
// Dependencies: {}
# [doc = " Returns the base page size, in bytes, of the current system."] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_getpagesize () -> i32 { BasePageSize :: SIZE . try_into () . unwrap () }
};
}
