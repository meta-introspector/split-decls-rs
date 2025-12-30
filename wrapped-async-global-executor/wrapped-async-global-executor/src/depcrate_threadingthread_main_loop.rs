// Generated macro for thread_main_loop (function)
macro_rules! Depcrate_threadingthread_main_loop {
() => {
// Module: crate::threading
// Provides: {"thread_main_loop"}
// Dependencies: {}
fn thread_main_loop () { let (s , r) = async_channel :: bounded (1) ; let (s_ack , r_ack) = async_channel :: bounded (1) ; THREAD_SHUTDOWN . with (| thread_shutdown | drop (thread_shutdown . set ((s , r_ack)))) ; loop { # [allow (clippy :: blocks_in_conditions)] if std :: panic :: catch_unwind (| | { crate :: executor :: LOCAL_EXECUTOR . with (| executor | { let local = executor . run (async { let _ = r . recv () . await ; }) ; let global = crate :: executor :: GLOBAL_EXECUTOR . run (future :: pending :: < () > ()) ; crate :: reactor :: block_on (future :: or (local , global)) ; }) ; }) . is_ok () { break ; } } wait_for_local_executor_completion () ; crate :: reactor :: block_on (async { let _ = s_ack . send (()) . await ; }) ; }
};
}
