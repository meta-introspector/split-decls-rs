// Generated macro for impl_28 (impl)
macro_rules! Depcrate_futures_03impl_28 {
() => {
// Module: crate::futures_03
// Provides: {"impl_28"}
// Dependencies: {}
impl < T > FromFutures < T > { # [doc = " Create a new adapter."] pub fn new (inner : T) -> Self { Self { inner } } # [doc = " Consume the adapter, returning the inner object."] pub fn into_inner (self) -> T { self . inner } }
};
}
