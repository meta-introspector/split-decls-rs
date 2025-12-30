// Generated macro for impl_41 (impl)
macro_rules! Depcrate_tokio_1impl_41 {
() => {
// Module: crate::tokio_1
// Provides: {"impl_41"}
// Dependencies: {}
impl < T > FromTokio < T > { # [doc = " Create a new adapter."] pub fn new (inner : T) -> Self { Self { inner } } # [doc = " Consume the adapter, returning the inner object."] pub fn into_inner (self) -> T { self . inner } }
};
}
