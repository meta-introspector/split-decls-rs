// Generated macro for impl_3245 (impl)
macro_rules! Depcrate_third_party_alga_alga_rotationimpl_3245 {
() => {
// Module: crate::third_party::alga::alga_rotation
// Provides: {"impl_3245"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > Transformation < Point < T , D > > for Rotation < T , D > { # [inline] fn transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . transform_point (pt) } # [inline] fn transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self . transform_vector (v) } }
};
}
