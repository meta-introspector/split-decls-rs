// Generated macro for sys_spawn2 (function)
macro_rules! Depcrate_syscalls_taskssys_spawn2 {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_spawn2"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_spawn2 (func : unsafe extern "C" fn (usize) , arg : usize , prio : u8 , stack_size : usize , selector : isize ,) -> Tid { unsafe { scheduler :: spawn (func , arg , Priority :: from (prio) , stack_size , selector) . into () } }
};
}
