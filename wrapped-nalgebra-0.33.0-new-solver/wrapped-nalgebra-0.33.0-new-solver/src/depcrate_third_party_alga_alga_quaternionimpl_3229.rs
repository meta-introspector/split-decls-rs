// Generated macro for impl_3229 (impl)
macro_rules! Depcrate_third_party_alga_alga_quaternionimpl_3229 {
() => {
// Module: crate::third_party::alga::alga_quaternion
// Provides: {"impl_3229"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > ProjectiveTransformation < Point3 < T > > for UnitQuaternion < T > { # [inline] fn inverse_transform_point (& self , pt : & Point3 < T >) -> Point3 < T > { self . inverse_transform_point (pt) } # [inline] fn inverse_transform_vector (& self , v : & Vector3 < T >) -> Vector3 < T > { self . inverse_transform_vector (v) } }
};
}
