// Generated macro for broadcast (function)
macro_rules! Depcrate_broadcastbroadcast {
() => {
// Module: crate::broadcast
// Provides: {"broadcast"}
// Dependencies: {}
# [doc = " Executes `op` within every thread in the current thread pool. If this is"] # [doc = " called from a non-Rayon thread, it will execute in the global thread pool."] # [doc = " Any attempts to use `join`, `scope`, or parallel iterators will then operate"] # [doc = " within that thread pool. When the call has completed on each thread, returns"] # [doc = " a vector containing all of their return values."] # [doc = ""] # [doc = " For more information, see the [`ThreadPool::broadcast()`] method."] # [doc = ""] # [doc = " [`ThreadPool::broadcast()`]: crate::ThreadPool::broadcast()"] pub fn broadcast < OP , R > (op : OP) -> Vec < R > where OP : Fn (BroadcastContext < '_ >) -> R + Sync , R : Send , { unsafe { broadcast_in (op , & Registry :: current ()) } }
};
}
