// Generated macro for transition (function)
macro_rules! Depcrate_google_cpu_profilertransition {
() => {
// Module: crate::google_cpu_profiler
// Provides: {"transition"}
// Dependencies: {}
fn transition (current : usize , new : usize) -> bool { static STATE : AtomicUsize = AtomicUsize :: new (OFF) ; STATE . compare_exchange (current , new , Ordering :: SeqCst , Ordering :: SeqCst) . is_ok () }
};
}
