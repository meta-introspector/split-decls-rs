// Generated macro for impl_3308 (impl)
macro_rules! Depcrate_third_party_alga_alga_unit_compleximpl_3308 {
() => {
// Module: crate::third_party::alga::alga_unit_complex
// Provides: {"impl_3308"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > TwoSidedInverse < Multiplicative > for UnitComplex < T > { # [inline] # [must_use = "Did you mean to use two_sided_inverse_mut()?"] fn two_sided_inverse (& self) -> Self { self . inverse () } # [inline] fn two_sided_inverse_mut (& mut self) { self . inverse_mut () } }
};
}
