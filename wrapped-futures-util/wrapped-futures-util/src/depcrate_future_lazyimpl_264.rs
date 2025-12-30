// Generated macro for impl_264 (impl)
macro_rules! Depcrate_future_lazyimpl_264 {
() => {
// Module: crate::future::lazy
// Provides: {"impl_264"}
// Dependencies: {}
impl < F , R > FusedFuture for Lazy < F > where F : FnOnce (& mut Context < '_ >) -> R , { fn is_terminated (& self) -> bool { self . f . is_none () } }
};
}
