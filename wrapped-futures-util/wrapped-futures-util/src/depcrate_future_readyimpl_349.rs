// Generated macro for impl_349 (impl)
macro_rules! Depcrate_future_readyimpl_349 {
() => {
// Module: crate::future::ready
// Provides: {"impl_349"}
// Dependencies: {}
impl < T > Ready < T > { # [doc = " Unwraps the value from this immediately ready future."] # [inline] pub fn into_inner (mut self) -> T { self . 0 . take () . unwrap () } }
};
}
