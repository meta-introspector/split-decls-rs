// Generated macro for impl_3295 (impl)
macro_rules! Depcrate_third_party_alga_alga_translationimpl_3295 {
() => {
// Module: crate::third_party::alga::alga_translation
// Provides: {"impl_3295"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > ProjectiveTransformation < Point < T , D > > for Translation < T , D > { # [inline] fn inverse_transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . inverse_transform_point (pt) } # [inline] fn inverse_transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { * v } }
};
}
