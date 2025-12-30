// Generated macro for impl_33 (impl)
macro_rules! Depcrate_local_poolimpl_33 {
() => {
// Module: crate::local_pool
// Provides: {"impl_33"}
// Dependencies: {}
impl < S : Stream + Unpin > BlockingStream < S > { # [doc = " Convert this `BlockingStream` into the inner `Stream` type."] pub fn into_inner (self) -> S { self . stream } }
};
}
