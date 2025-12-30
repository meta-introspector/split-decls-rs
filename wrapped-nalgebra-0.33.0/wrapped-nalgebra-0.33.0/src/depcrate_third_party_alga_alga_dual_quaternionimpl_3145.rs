// Generated macro for impl_3145 (impl)
macro_rules! Depcrate_third_party_alga_alga_dual_quaternionimpl_3145 {
() => {
// Module: crate::third_party::alga::alga_dual_quaternion
// Provides: {"impl_3145"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > ProjectiveTransformation < Point3 < T > > for UnitDualQuaternion < T > { # [inline] fn inverse_transform_point (& self , pt : & Point3 < T >) -> Point3 < T > { self . inverse_transform_point (pt) } # [inline] fn inverse_transform_vector (& self , v : & Vector3 < T >) -> Vector3 < T > { self . inverse_transform_vector (v) } }
};
}
