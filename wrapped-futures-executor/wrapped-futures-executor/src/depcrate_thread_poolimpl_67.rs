// Generated macro for impl_67 (impl)
macro_rules! Depcrate_thread_poolimpl_67 {
() => {
// Module: crate::thread_pool
// Provides: {"impl_67"}
// Dependencies: {}
impl Drop for ThreadPool { fn drop (& mut self) { if self . state . cnt . fetch_sub (1 , Ordering :: Relaxed) == 1 { for _ in 0 .. self . state . size { self . state . send (Message :: Close) ; } } } }
};
}
