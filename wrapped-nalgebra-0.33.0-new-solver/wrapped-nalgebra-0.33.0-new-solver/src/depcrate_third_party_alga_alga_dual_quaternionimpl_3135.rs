// Generated macro for impl_3135 (impl)
macro_rules! Depcrate_third_party_alga_alga_dual_quaternionimpl_3135 {
() => {
// Module: crate::third_party::alga::alga_dual_quaternion
// Provides: {"impl_3135"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > AbstractModule for DualQuaternion < T > { type AbstractRing = T ; # [inline] fn multiply_by (& self , n : T) -> Self { self * n } }
};
}
