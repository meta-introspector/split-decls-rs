// Generated macro for tests (module)
macro_rules! Depcrate_task_panic_wakertests {
() => {
// Module: crate::task::panic_waker
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] # [should_panic (expected = "should not be woken")] fn issue_2091_cross_thread_segfault () { let waker = std :: thread :: spawn (super :: panic_waker_ref) . join () . unwrap () ; waker . wake_by_ref () ; } }
};
}
