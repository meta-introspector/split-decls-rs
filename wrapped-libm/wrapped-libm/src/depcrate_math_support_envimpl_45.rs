// Generated macro for impl_45 (impl)
macro_rules! Depcrate_math_support_envimpl_45 {
() => {
// Module: crate::math::support::env
// Provides: {"impl_45"}
// Dependencies: {}
impl < T > FpResult < T > { pub fn new (val : T , status : Status) -> Self { Self { val , status } } # [doc = " Return `val` with `Status::OK`."] pub fn ok (val : T) -> Self { Self { val , status : Status :: OK , } } }
};
}
