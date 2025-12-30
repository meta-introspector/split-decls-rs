// Generated macro for impl_183 (impl)
macro_rules! Depcrate_future_future_sharedimpl_183 {
() => {
// Module: crate::future::future::shared
// Provides: {"impl_183"}
// Dependencies: {}
impl < Fut > Clone for Shared < Fut > where Fut : Future , { fn clone (& self) -> Self { Self { inner : self . inner . clone () , waker_key : NULL_WAKER_KEY } } }
};
}
