// Generated macro for sys_notify (function)
macro_rules! Depcrate_syscalls_condvarsys_notify {
() => {
// Module: crate::syscalls::condvar
// Provides: {"sys_notify"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_notify (ptr : usize , count : i32) -> i32 { unsafe { let id = ptr :: with_exposed_provenance :: < usize > (ptr) ; if id . is_null () { debug ! ("sys_notify: invalid address to condition variable") ; return - 1 ; } if * id == 0 { debug ! ("sys_notify: invalid reference to condition variable") ; return - 1 ; } let cond = & mut * (ptr :: with_exposed_provenance_mut :: < CondQueue > (* id)) ; if count < 0 { while cond . counter . load (Ordering :: SeqCst) > 0 { cond . counter . fetch_sub (1 , Ordering :: SeqCst) ; cond . sem1 . release () ; cond . sem2 . acquire (None) ; } } else { for _ in 0 .. count { cond . counter . fetch_sub (1 , Ordering :: SeqCst) ; cond . sem1 . release () ; cond . sem2 . acquire (None) ; } } 0 } }
};
}
