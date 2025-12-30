// Generated macro for sys_setitimer (function)
macro_rules! Depcrate_syscalls_timersys_setitimer {
() => {
// Module: crate::syscalls::timer
// Provides: {"sys_setitimer"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_setitimer (_which : i32 , _value : * const itimerval , _ovalue : * mut itimerval ,) -> i32 { debug ! ("Called sys_setitimer, which is unimplemented and always returns 0") ; 0 }
};
}
