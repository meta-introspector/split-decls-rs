// Generated macro for impl_9 (impl)
macro_rules! Depcrate_stdimpl_9 {
() => {
// Module: crate::std
// Provides: {"impl_9"}
// Dependencies: {}
impl < T > FromStd < T > { # [doc = " Create a new adapter."] pub fn new (inner : T) -> Self { Self { inner } } # [doc = " Consume the adapter, returning the inner object."] pub fn into_inner (self) -> T { self . inner } }
};
}
