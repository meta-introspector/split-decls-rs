// Generated macro for impl_3157 (impl)
macro_rules! Depcrate_third_party_alga_alga_isometryimpl_3157 {
() => {
// Module: crate::third_party::alga::alga_isometry
// Provides: {"impl_3157"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > TwoSidedInverse < Multiplicative > for Isometry < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { # [inline] # [must_use = "Did you mean to use two_sided_inverse_mut()?"] fn two_sided_inverse (& self) -> Self { self . inverse () } # [inline] fn two_sided_inverse_mut (& mut self) { self . inverse_mut () } }
};
}
