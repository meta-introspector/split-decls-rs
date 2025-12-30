// Generated macro for impl_3241 (impl)
macro_rules! Depcrate_third_party_alga_alga_rotationimpl_3241 {
() => {
// Module: crate::third_party::alga::alga_rotation
// Provides: {"impl_3241"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > TwoSidedInverse < Multiplicative > for Rotation < T , D > { # [inline] # [must_use = "Did you mean to use two_sided_inverse_mut()?"] fn two_sided_inverse (& self) -> Self { self . transpose () } # [inline] fn two_sided_inverse_mut (& mut self) { self . transpose_mut () } }
};
}
