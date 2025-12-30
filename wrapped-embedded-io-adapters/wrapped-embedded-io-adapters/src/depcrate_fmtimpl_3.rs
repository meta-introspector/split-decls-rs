// Generated macro for impl_3 (impl)
macro_rules! Depcrate_fmtimpl_3 {
() => {
// Module: crate::fmt
// Provides: {"impl_3"}
// Dependencies: {}
impl < T > ToFmt < T > { # [doc = " Create a new adapter."] pub fn new (inner : T) -> Self { Self { inner } } # [doc = " Consume the adapter, returning the inner object."] pub fn into_inner (self) -> T { self . inner } }
};
}
