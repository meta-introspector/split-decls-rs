// Generated macro for impl_17 (impl)
macro_rules! Depcrate_stdimpl_17 {
() => {
// Module: crate::std
// Provides: {"impl_17"}
// Dependencies: {}
impl < T > ToStd < T > { # [doc = " Create a new adapter."] pub fn new (inner : T) -> Self { Self { inner } } # [doc = " Consume the adapter, returning the inner object."] pub fn into_inner (self) -> T { self . inner } }
};
}
