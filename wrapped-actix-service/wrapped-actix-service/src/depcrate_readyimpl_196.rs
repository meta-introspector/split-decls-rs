// Generated macro for impl_196 (impl)
macro_rules! Depcrate_readyimpl_196 {
() => {
// Module: crate::ready
// Provides: {"impl_196"}
// Dependencies: {}
impl < T > Ready < T > { # [doc = " Unwraps the value from this immediately ready future."] # [inline] pub fn into_inner (mut self) -> T { self . val . take () . unwrap () } }
};
}
