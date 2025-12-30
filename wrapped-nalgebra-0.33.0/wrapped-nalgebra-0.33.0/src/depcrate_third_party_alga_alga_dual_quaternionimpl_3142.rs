// Generated macro for impl_3142 (impl)
macro_rules! Depcrate_third_party_alga_alga_dual_quaternionimpl_3142 {
() => {
// Module: crate::third_party::alga::alga_dual_quaternion
// Provides: {"impl_3142"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > TwoSidedInverse < Multiplicative > for UnitDualQuaternion < T > { # [inline] fn two_sided_inverse (& self) -> Self { self . inverse () } # [inline] fn two_sided_inverse_mut (& mut self) { self . inverse_mut () } }
};
}
