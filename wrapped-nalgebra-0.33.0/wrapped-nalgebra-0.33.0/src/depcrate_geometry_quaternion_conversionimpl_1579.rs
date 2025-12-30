// Generated macro for impl_1579 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1579 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1579"}
// Dependencies: {}
impl < T1 : RealField , T2 : RealField + SupersetOf < T1 > > SubsetOf < Matrix4 < T2 > > for UnitQuaternion < T1 > { # [inline] fn to_superset (& self) -> Matrix4 < T2 > { self . clone () . to_homogeneous () . to_superset () } # [inline] fn is_in_subset (m : & Matrix4 < T2 >) -> bool { crate :: is_convertible :: < _ , Rotation3 < T1 > > (m) } # [inline] fn from_superset_unchecked (m : & Matrix4 < T2 >) -> Self { let rot : Rotation3 < T1 > = crate :: convert_ref_unchecked (m) ; Self :: from_rotation_matrix (& rot) } }
};
}
