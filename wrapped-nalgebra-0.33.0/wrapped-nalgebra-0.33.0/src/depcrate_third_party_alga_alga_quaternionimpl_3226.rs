// Generated macro for impl_3226 (impl)
macro_rules! Depcrate_third_party_alga_alga_quaternionimpl_3226 {
() => {
// Module: crate::third_party::alga::alga_quaternion
// Provides: {"impl_3226"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > TwoSidedInverse < Multiplicative > for UnitQuaternion < T > { # [inline] fn two_sided_inverse (& self) -> Self { self . inverse () } # [inline] fn two_sided_inverse_mut (& mut self) { self . inverse_mut () } }
};
}
