// Generated macro for sys_add_queue (function)
macro_rules! Depcrate_syscalls_condvarsys_add_queue {
() => {
// Module: crate::syscalls::condvar
// Provides: {"sys_add_queue"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_add_queue (ptr : usize , timeout_ns : i64) -> i32 { unsafe { let id = ptr :: with_exposed_provenance_mut :: < usize > (ptr) ; if id . is_null () { debug ! ("sys_add_queue: invalid address to condition variable") ; return - 1 ; } if * id == 0 { debug ! ("Create condition variable queue") ; let queue = Box :: new (CondQueue :: new ()) ; * id = Box :: into_raw (queue) as usize ; } if timeout_ns <= 0 { let cond = & mut * (ptr :: with_exposed_provenance_mut :: < CondQueue > (* id)) ; cond . counter . fetch_add (1 , Ordering :: SeqCst) ; 0 } else { error ! ("Conditional variables with timeout is currently not supported") ; - 1 } } }
};
}
