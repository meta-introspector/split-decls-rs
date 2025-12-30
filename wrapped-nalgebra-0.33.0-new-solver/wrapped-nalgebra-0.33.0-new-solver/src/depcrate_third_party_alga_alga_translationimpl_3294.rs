// Generated macro for impl_3294 (impl)
macro_rules! Depcrate_third_party_alga_alga_translationimpl_3294 {
() => {
// Module: crate::third_party::alga::alga_translation
// Provides: {"impl_3294"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > Transformation < Point < T , D > > for Translation < T , D > { # [inline] fn transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . transform_point (pt) } # [inline] fn transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { * v } }
};
}
