// Generated macro for current_thread_has_pending_tasks (function)
macro_rules! Depcrate_thread_poolcurrent_thread_has_pending_tasks {
() => {
// Module: crate::thread_pool
// Provides: {"current_thread_has_pending_tasks"}
// Dependencies: {}
# [doc = " If called from a Rayon worker thread, indicates whether that"] # [doc = " thread's local deque still has pending tasks. Otherwise, returns"] # [doc = " `None`. For more information, see [the"] # [doc = " `ThreadPool::current_thread_has_pending_tasks()` method][m]."] # [doc = ""] # [doc = " [m]: ThreadPool::current_thread_has_pending_tasks()"] # [inline] pub fn current_thread_has_pending_tasks () -> Option < bool > { unsafe { let curr = WorkerThread :: current () . as_ref () ? ; Some (! curr . local_deque_is_empty ()) } }
};
}
