// Generated macro for current_thread_id (function)
macro_rules! Depcrate_wakercurrent_thread_id {
() => {
// Module: crate::waker
// Provides: {"current_thread_id"}
// Dependencies: {}
# [doc = " Returns the id of the current thread."] # [inline] fn current_thread_id () -> ThreadId { std :: thread_local ! { # [doc = " Cached thread-local id."] static THREAD_ID : ThreadId = thread :: current () . id () ; } THREAD_ID . try_with (| id | * id) . unwrap_or_else (| _ | thread :: current () . id ()) }
};
}
