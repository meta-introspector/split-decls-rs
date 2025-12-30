// Generated macro for impl_3264 (impl)
macro_rules! Depcrate_third_party_alga_alga_similarityimpl_3264 {
() => {
// Module: crate::third_party::alga::alga_similarity
// Provides: {"impl_3264"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > ProjectiveTransformation < Point < T , D > > for Similarity < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { # [inline] fn inverse_transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . inverse_transform_point (pt) } # [inline] fn inverse_transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self . inverse_transform_vector (v) } }
};
}
