// Generated macro for impl_1573 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1573 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1573"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < UnitQuaternion < T2 > > for UnitQuaternion < T1 > where T1 : Scalar , T2 : Scalar + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> UnitQuaternion < T2 > { UnitQuaternion :: new_unchecked (self . as_ref () . to_superset ()) } # [inline] fn is_in_subset (uq : & UnitQuaternion < T2 >) -> bool { crate :: is_convertible :: < _ , Quaternion < T1 > > (uq . as_ref ()) } # [inline] fn from_superset_unchecked (uq : & UnitQuaternion < T2 >) -> Self { Self :: new_unchecked (crate :: convert_ref_unchecked (uq . as_ref ())) } }
};
}
