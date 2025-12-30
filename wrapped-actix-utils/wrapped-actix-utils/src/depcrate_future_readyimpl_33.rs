// Generated macro for impl_33 (impl)
macro_rules! Depcrate_future_readyimpl_33 {
() => {
// Module: crate::future::ready
// Provides: {"impl_33"}
// Dependencies: {}
impl < T > Ready < T > { # [doc = " Unwraps the value from this immediately ready future."] # [inline] pub fn into_inner (mut self) -> T { self . val . take () . unwrap () } }
};
}
