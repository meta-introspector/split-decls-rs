// Generated macro for impl_3290 (impl)
macro_rules! Depcrate_third_party_alga_alga_translationimpl_3290 {
() => {
// Module: crate::third_party::alga::alga_translation
// Provides: {"impl_3290"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > TwoSidedInverse < Multiplicative > for Translation < T , D > { # [inline] # [must_use = "Did you mean to use two_sided_inverse_mut()?"] fn two_sided_inverse (& self) -> Self { self . inverse () } # [inline] fn two_sided_inverse_mut (& mut self) { self . inverse_mut () } }
};
}
