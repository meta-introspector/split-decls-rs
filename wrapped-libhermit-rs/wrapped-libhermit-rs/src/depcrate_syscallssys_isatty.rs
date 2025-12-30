// Generated macro for sys_isatty (function)
macro_rules! Depcrate_syscallssys_isatty {
() => {
// Module: crate::syscalls
// Provides: {"sys_isatty"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_isatty (fd : i32) -> i32 { match isatty (fd) { Err (e) => - i32 :: from (e) , Ok (v) => { if v { 1 } else { 0 } } } }
};
}
