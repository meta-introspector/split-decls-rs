// Generated macro for current (function)
macro_rules! Depcrate_threadcurrent {
() => {
// Module: crate::thread
// Provides: {"current"}
// Dependencies: {}
# [doc = " Returns a handle to the current thread."] pub fn current () -> Thread { rt :: execution (| execution | { let thread = execution . threads . local (& CURRENT_THREAD_KEY) ; if let Some (thread) = thread { thread . unwrap () . clone () } else { init_current (execution , None) } }) }
};
}
