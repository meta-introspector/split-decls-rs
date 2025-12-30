// Generated macro for impl_3228 (impl)
macro_rules! Depcrate_third_party_alga_alga_quaternionimpl_3228 {
() => {
// Module: crate::third_party::alga::alga_quaternion
// Provides: {"impl_3228"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > Transformation < Point3 < T > > for UnitQuaternion < T > { # [inline] fn transform_point (& self , pt : & Point3 < T >) -> Point3 < T > { self . transform_point (pt) } # [inline] fn transform_vector (& self , v : & Vector3 < T >) -> Vector3 < T > { self . transform_vector (v) } }
};
}
