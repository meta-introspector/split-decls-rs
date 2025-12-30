// Generated macro for stop_thread (function)
macro_rules! Depcrate_threadingstop_thread {
() => {
// Module: crate::threading
// Provides: {"stop_thread"}
// Dependencies: {}
# [doc = " Stop one of the executor threads, down to configured min value"] # [doc = ""] # [doc = " Returns whether a thread has been stopped."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " async_global_executor::stop_thread();"] # [doc = " ```"] pub fn stop_thread () -> Task < bool > { crate :: spawn (stop_current_executor_thread ()) }
};
}
