// Generated macro for impl_828 (impl)
macro_rules! Depcrate_executor_taskimpl_828 {
() => {
// Module: crate::executor::task
// Provides: {"impl_828"}
// Dependencies: {}
impl AsyncTaskId { fn new () -> Self { static NEXT_ID : AtomicU32 = AtomicU32 :: new (0) ; AsyncTaskId (NEXT_ID . fetch_add (1 , Ordering :: Relaxed)) } }
};
}
