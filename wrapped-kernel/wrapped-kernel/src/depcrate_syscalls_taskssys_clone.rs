// Generated macro for sys_clone (function)
macro_rules! Depcrate_syscalls_taskssys_clone {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_clone"}
// Dependencies: {}
# [doc = " Creates a new thread based on the configuration of the current thread."] # [cfg (feature = "newlib")] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_clone (id : * mut Tid , func : extern "C" fn (usize) , arg : usize) -> i32 { let task_id = core_scheduler () . clone (func , arg) ; if ! id . is_null () { unsafe { * id = task_id . into () ; } } 0 }
};
}
