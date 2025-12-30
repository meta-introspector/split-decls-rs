// Generated macro for in_worker (function)
macro_rules! Depcrate_registryin_worker {
() => {
// Module: crate::registry
// Provides: {"in_worker"}
// Dependencies: {}
# [doc = " If already in a worker-thread, just execute `op`.  Otherwise,"] # [doc = " execute `op` in the default thread pool. Either way, block until"] # [doc = " `op` completes and return its return value. If `op` panics, that"] # [doc = " panic will be propagated as well.  The second argument indicates"] # [doc = " `true` if injection was performed, `false` if executed directly."] pub (super) fn in_worker < OP , R > (op : OP) -> R where OP : FnOnce (& WorkerThread , bool) -> R + Send , R : Send , { unsafe { let owner_thread = WorkerThread :: current () ; if ! owner_thread . is_null () { op (& * owner_thread , false) } else { global_registry () . in_worker (op) } } }
};
}
