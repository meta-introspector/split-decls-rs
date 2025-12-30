// Generated macro for impl_3246 (impl)
macro_rules! Depcrate_third_party_alga_alga_rotationimpl_3246 {
() => {
// Module: crate::third_party::alga::alga_rotation
// Provides: {"impl_3246"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > ProjectiveTransformation < Point < T , D > > for Rotation < T , D > { # [inline] fn inverse_transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . inverse_transform_point (pt) } # [inline] fn inverse_transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self . inverse_transform_vector (v) } }
};
}
