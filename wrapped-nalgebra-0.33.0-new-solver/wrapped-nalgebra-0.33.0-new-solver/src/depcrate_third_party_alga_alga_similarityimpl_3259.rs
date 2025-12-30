// Generated macro for impl_3259 (impl)
macro_rules! Depcrate_third_party_alga_alga_similarityimpl_3259 {
() => {
// Module: crate::third_party::alga::alga_similarity
// Provides: {"impl_3259"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > TwoSidedInverse < Multiplicative > for Similarity < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { # [inline] # [must_use = "Did you mean to use two_sided_inverse_mut()?"] fn two_sided_inverse (& self) -> Self { self . inverse () } # [inline] fn two_sided_inverse_mut (& mut self) { self . inverse_mut () } }
};
}
