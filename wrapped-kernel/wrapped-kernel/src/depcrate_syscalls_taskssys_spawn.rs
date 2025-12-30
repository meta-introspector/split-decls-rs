// Generated macro for sys_spawn (function)
macro_rules! Depcrate_syscalls_taskssys_spawn {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_spawn"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_spawn (id : * mut Tid , func : unsafe extern "C" fn (usize) , arg : usize , prio : u8 , selector : isize ,) -> i32 { let new_id = unsafe { scheduler :: spawn (func , arg , Priority :: from (prio) , USER_STACK_SIZE , selector) . into () } ; if ! id . is_null () { unsafe { * id = new_id ; } } 0 }
};
}
