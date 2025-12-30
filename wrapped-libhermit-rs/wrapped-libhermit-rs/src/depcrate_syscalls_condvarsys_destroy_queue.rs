// Generated macro for sys_destroy_queue (function)
macro_rules! Depcrate_syscalls_condvarsys_destroy_queue {
() => {
// Module: crate::syscalls::condvar
// Provides: {"sys_destroy_queue"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_destroy_queue (ptr : usize) -> i32 { unsafe { let id = ptr :: with_exposed_provenance_mut :: < usize > (ptr) ; if id . is_null () { debug ! ("sys_wait: invalid address to condition variable") ; return - 1 ; } if * id != 0 { let cond = Box :: from_raw (ptr :: with_exposed_provenance_mut :: < CondQueue > (* id)) ; mem :: drop (cond) ; } 0 } }
};
}
