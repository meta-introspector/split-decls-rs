// Generated macro for impl_3234 (impl)
macro_rules! Depcrate_third_party_alga_alga_quaternionimpl_3234 {
() => {
// Module: crate::third_party::alga::alga_quaternion
// Provides: {"impl_3234"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > Rotation < Point3 < T > > for UnitQuaternion < T > { # [inline] fn powf (& self , n : T) -> Option < Self > { Some (self . powf (n)) } # [inline] fn rotation_between (a : & Vector3 < T > , b : & Vector3 < T >) -> Option < Self > { Self :: rotation_between (a , b) } # [inline] fn scaled_rotation_between (a : & Vector3 < T > , b : & Vector3 < T > , s : T) -> Option < Self > { Self :: scaled_rotation_between (a , b , s) } }
};
}
