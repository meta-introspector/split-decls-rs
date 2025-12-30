// Generated macro for Inner (struct)
macro_rules! Depcrate_work_queueInner {
() => {
// Module: crate::work_queue
// Provides: {"Inner"}
// Dependencies: {}
struct Inner < T > { queue : Mutex < VecDeque < T > > , condvar : Condvar , closed : AtomicBool , }
};
}
