// Generated macro for impl_3311 (impl)
macro_rules! Depcrate_third_party_alga_alga_unit_compleximpl_3311 {
() => {
// Module: crate::third_party::alga::alga_unit_complex
// Provides: {"impl_3311"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > Transformation < Point2 < T > > for UnitComplex < T > { # [inline] fn transform_point (& self , pt : & Point2 < T >) -> Point2 < T > { self . transform_point (pt) } # [inline] fn transform_vector (& self , v : & Vector2 < T >) -> Vector2 < T > { self . transform_vector (v) } }
};
}
