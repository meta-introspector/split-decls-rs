// Generated macro for PoolState (struct)
macro_rules! Depcrate_thread_poolPoolState {
() => {
// Module: crate::thread_pool
// Provides: {"PoolState"}
// Dependencies: {}
struct PoolState { tx : Mutex < mpsc :: Sender < Message > > , rx : Mutex < mpsc :: Receiver < Message > > , cnt : AtomicUsize , size : usize , }
};
}
