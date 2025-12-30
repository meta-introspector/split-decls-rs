// Generated macro for impl_1574 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1574 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1574"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < Rotation < T2 , 3 > > for UnitQuaternion < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> Rotation3 < T2 > { let q : UnitQuaternion < T2 > = self . to_superset () ; q . to_rotation_matrix () } # [inline] fn is_in_subset (rot : & Rotation3 < T2 >) -> bool { crate :: is_convertible :: < _ , Rotation3 < T1 > > (rot) } # [inline] fn from_superset_unchecked (rot : & Rotation3 < T2 >) -> Self { let q = UnitQuaternion :: < T2 > :: from_rotation_matrix (rot) ; crate :: convert_unchecked (q) } }
};
}
