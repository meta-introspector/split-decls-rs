// Generated macro for impl_3144 (impl)
macro_rules! Depcrate_third_party_alga_alga_dual_quaternionimpl_3144 {
() => {
// Module: crate::third_party::alga::alga_dual_quaternion
// Provides: {"impl_3144"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > Transformation < Point3 < T > > for UnitDualQuaternion < T > { # [inline] fn transform_point (& self , pt : & Point3 < T >) -> Point3 < T > { self . transform_point (pt) } # [inline] fn transform_vector (& self , v : & Vector3 < T >) -> Vector3 < T > { self . transform_vector (v) } }
};
}
