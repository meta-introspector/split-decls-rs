// Generated macro for impl_271 (impl)
macro_rules! Depcrate_math_support_int_traitsimpl_271 {
() => {
// Module: crate::math::support::int_traits
// Provides: {"impl_271"}
// Dependencies: {}
impl < T : Copy , U : CastInto < T > + Copy > CastFrom < U > for T { fn cast_from (value : U) -> Self { value . cast () } fn cast_from_lossy (value : U) -> Self { value . cast_lossy () } }
};
}
