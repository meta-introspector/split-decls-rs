// Generated macro for impl_188 (impl)
macro_rules! Depcrate_math_support_envimpl_188 {
() => {
// Module: crate::math::support::env
// Provides: {"impl_188"}
// Dependencies: {}
impl < T > FpResult < T > { pub fn new (val : T , status : Status) -> Self { Self { val , status } } # [doc = " Return `val` with `Status::OK`."] pub fn ok (val : T) -> Self { Self { val , status : Status :: OK , } } }
};
}
