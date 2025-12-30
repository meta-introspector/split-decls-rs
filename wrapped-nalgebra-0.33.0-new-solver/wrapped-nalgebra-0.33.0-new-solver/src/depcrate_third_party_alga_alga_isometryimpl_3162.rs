// Generated macro for impl_3162 (impl)
macro_rules! Depcrate_third_party_alga_alga_isometryimpl_3162 {
() => {
// Module: crate::third_party::alga::alga_isometry
// Provides: {"impl_3162"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > ProjectiveTransformation < Point < T , D > > for Isometry < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { # [inline] fn inverse_transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . inverse_transform_point (pt) } # [inline] fn inverse_transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self . inverse_transform_vector (v) } }
};
}
