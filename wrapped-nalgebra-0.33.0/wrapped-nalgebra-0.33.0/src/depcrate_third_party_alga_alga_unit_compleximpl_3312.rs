// Generated macro for impl_3312 (impl)
macro_rules! Depcrate_third_party_alga_alga_unit_compleximpl_3312 {
() => {
// Module: crate::third_party::alga::alga_unit_complex
// Provides: {"impl_3312"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > ProjectiveTransformation < Point2 < T > > for UnitComplex < T > { # [inline] fn inverse_transform_point (& self , pt : & Point2 < T >) -> Point2 < T > { self . inverse_transform_point (pt) } # [inline] fn inverse_transform_vector (& self , v : & Vector2 < T >) -> Vector2 < T > { self . inverse_transform_vector (v) } }
};
}
