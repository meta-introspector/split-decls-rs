// Generated macro for impl_66 (impl)
macro_rules! Depcrate_thread_poolimpl_66 {
() => {
// Module: crate::thread_pool
// Provides: {"impl_66"}
// Dependencies: {}
impl Clone for ThreadPool { fn clone (& self) -> Self { self . state . cnt . fetch_add (1 , Ordering :: Relaxed) ; Self { state : self . state . clone () } } }
};
}
