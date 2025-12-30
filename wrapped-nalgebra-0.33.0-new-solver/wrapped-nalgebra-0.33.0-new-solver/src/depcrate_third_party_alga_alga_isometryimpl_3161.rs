// Generated macro for impl_3161 (impl)
macro_rules! Depcrate_third_party_alga_alga_isometryimpl_3161 {
() => {
// Module: crate::third_party::alga::alga_isometry
// Provides: {"impl_3161"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > Transformation < Point < T , D > > for Isometry < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { # [inline] fn transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . transform_point (pt) } # [inline] fn transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self . transform_vector (v) } }
};
}
