// Generated macro for Inner (struct)
macro_rules! Depcrate_sync_parkerInner {
() => {
// Module: crate::sync::parker
// Provides: {"Inner"}
// Dependencies: {}
struct Inner { state : AtomicUsize , lock : Mutex < () > , cvar : Condvar , }
};
}
