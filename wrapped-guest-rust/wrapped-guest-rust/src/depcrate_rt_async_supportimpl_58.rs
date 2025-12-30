// Generated macro for impl_58 (impl)
macro_rules! Depcrate_rt_async_supportimpl_58 {
() => {
// Module: crate::rt::async_support
// Provides: {"impl_58"}
// Dependencies: {}
impl Wake for FutureWaker { fn wake (self : Arc < Self >) { Self :: wake_by_ref (& self) } fn wake_by_ref (self : & Arc < Self >) { self . 0 . store (true , Ordering :: Relaxed) } }
};
}
