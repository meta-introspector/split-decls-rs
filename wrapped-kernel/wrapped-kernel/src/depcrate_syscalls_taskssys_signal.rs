// Generated macro for sys_signal (function)
macro_rules! Depcrate_syscalls_taskssys_signal {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_signal"}
// Dependencies: {}
# [cfg (feature = "newlib")] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_signal (_handler : SignalHandler) -> i32 { debug ! ("sys_signal is unimplemented") ; 0 }
};
}
