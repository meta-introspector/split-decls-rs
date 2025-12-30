// Generated macro for impl_1576 (impl)
macro_rules! Depcrate_geometry_quaternion_conversionimpl_1576 {
() => {
// Module: crate::geometry::quaternion_conversion
// Provides: {"impl_1576"}
// Dependencies: {}
impl < T1 , T2 > SubsetOf < UnitDualQuaternion < T2 > > for UnitQuaternion < T1 > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , { # [inline] fn to_superset (& self) -> UnitDualQuaternion < T2 > { let q : UnitQuaternion < T2 > = crate :: convert_ref (self) ; UnitDualQuaternion :: from_rotation (q) } # [inline] fn is_in_subset (dq : & UnitDualQuaternion < T2 >) -> bool { dq . translation () . vector . is_zero () } # [inline] fn from_superset_unchecked (dq : & UnitDualQuaternion < T2 >) -> Self { crate :: convert_unchecked (dq . rotation ()) } }
};
}
