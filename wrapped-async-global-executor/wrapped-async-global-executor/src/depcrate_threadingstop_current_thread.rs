// Generated macro for stop_current_thread (function)
macro_rules! Depcrate_threadingstop_current_thread {
() => {
// Module: crate::threading
// Provides: {"stop_current_thread"}
// Dependencies: {}
# [doc = " Stop the current executor thread, if we exceed the configured min value"] # [doc = ""] # [doc = " Returns whether the thread has been stopped."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " async_global_executor::stop_current_thread();"] # [doc = " ```"] pub fn stop_current_thread () -> Task < bool > { crate :: spawn_local (stop_current_executor_thread ()) }
};
}
