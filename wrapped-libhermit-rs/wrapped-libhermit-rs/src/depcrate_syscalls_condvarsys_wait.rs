// Generated macro for sys_wait (function)
macro_rules! Depcrate_syscalls_condvarsys_wait {
() => {
// Module: crate::syscalls::condvar
// Provides: {"sys_wait"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_wait (ptr : usize) -> i32 { unsafe { let id = ptr :: with_exposed_provenance_mut :: < usize > (ptr) ; if id . is_null () { debug ! ("sys_wait: invalid address to condition variable") ; return - 1 ; } if * id == 0 { error ! ("sys_wait: Unable to determine condition variable") ; return - 1 ; } let cond = & mut * (ptr :: with_exposed_provenance_mut :: < CondQueue > (* id)) ; cond . sem1 . acquire (None) ; cond . sem2 . release () ; 0 } }
};
}
