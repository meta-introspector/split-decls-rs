// Generated macro for sys_init_queue (function)
macro_rules! Depcrate_syscalls_condvarsys_init_queue {
() => {
// Module: crate::syscalls::condvar
// Provides: {"sys_init_queue"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_init_queue (ptr : usize) -> i32 { unsafe { let id = ptr :: with_exposed_provenance_mut :: < usize > (ptr) ; if id . is_null () { debug ! ("sys_init_queue: invalid address to condition variable") ; return - 1 ; } if * id == 0 { debug ! ("Create condition variable queue") ; let queue = Box :: new (CondQueue :: new ()) ; * id = Box :: into_raw (queue) as usize ; } 0 } }
};
}
