// Generated macro for impl_3260 (impl)
macro_rules! Depcrate_third_party_alga_alga_similarityimpl_3260 {
() => {
// Module: crate::third_party::alga::alga_similarity
// Provides: {"impl_3260"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > AbstractMagma < Multiplicative > for Similarity < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { # [inline] fn operate (& self , rhs : & Self) -> Self { self * rhs } }
};
}
