// Generated macro for impl_3258 (impl)
macro_rules! Depcrate_third_party_alga_alga_similarityimpl_3258 {
() => {
// Module: crate::third_party::alga::alga_similarity
// Provides: {"impl_3258"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > Identity < Multiplicative > for Similarity < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { # [inline] fn identity () -> Self { Self :: identity () } }
};
}
