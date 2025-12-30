// Generated macro for wait_for_local_executor_completion (function)
macro_rules! Depcrate_threadingwait_for_local_executor_completion {
() => {
// Module: crate::threading
// Provides: {"wait_for_local_executor_completion"}
// Dependencies: {}
fn wait_for_local_executor_completion () { loop { # [allow (clippy :: blocks_in_conditions)] if std :: panic :: catch_unwind (| | { crate :: executor :: LOCAL_EXECUTOR . with (| executor | { crate :: reactor :: block_on (async { while ! executor . is_empty () { executor . tick () . await ; } }) ; }) ; }) . is_ok () { break ; } } }
};
}
