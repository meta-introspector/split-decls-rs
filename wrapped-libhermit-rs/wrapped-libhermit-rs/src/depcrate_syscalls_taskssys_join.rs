// Generated macro for sys_join (function)
macro_rules! Depcrate_syscalls_taskssys_join {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_join"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_join (id : Tid) -> i32 { match scheduler :: join (TaskId :: from (id)) { Ok (()) => 0 , _ => - i32 :: from (Errno :: Inval) , } }
};
}
