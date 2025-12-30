// Generated macro for MutexGuardArc (struct)
macro_rules! Depcrate_mutexMutexGuardArc {
() => {
// Module: crate::mutex
// Provides: {"MutexGuardArc"}
// Dependencies: {}
# [doc = " An owned guard that releases the mutex when dropped."] # [clippy :: has_significant_drop] pub struct MutexGuardArc < T : ? Sized > (Arc < Mutex < T > >) ;
};
}
