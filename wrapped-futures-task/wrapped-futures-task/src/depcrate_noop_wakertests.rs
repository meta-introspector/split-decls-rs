// Generated macro for tests (module)
macro_rules! Depcrate_noop_wakertests {
() => {
// Module: crate::noop_waker
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] # [cfg (feature = "std")] fn issue_2091_cross_thread_segfault () { let waker = std :: thread :: spawn (super :: noop_waker_ref) . join () . unwrap () ; waker . wake_by_ref () ; } }
};
}
