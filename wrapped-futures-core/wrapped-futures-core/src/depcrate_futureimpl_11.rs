// Generated macro for impl_11 (impl)
macro_rules! Depcrate_futureimpl_11 {
() => {
// Module: crate::future
// Provides: {"impl_11"}
// Dependencies: {}
impl < F : FusedFuture + ? Sized + Unpin > FusedFuture for & mut F { fn is_terminated (& self) -> bool { < F as FusedFuture > :: is_terminated (& * * self) } }
};
}
