// Generated macro for impl_1316 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1316 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1316"}
// Dependencies: {}
impl < T1 , T2 , D : DimName > SubsetOf < OPoint < T2 , D > > for OPoint < T1 , D > where T1 : Scalar , T2 : Scalar + SupersetOf < T1 > , DefaultAllocator : Allocator < D > , { # [inline] fn to_superset (& self) -> OPoint < T2 , D > { OPoint :: from (self . coords . to_superset ()) } # [inline] fn is_in_subset (m : & OPoint < T2 , D >) -> bool { m . iter () . all (| e | e . is_in_subset ()) } # [inline] fn from_superset_unchecked (m : & OPoint < T2 , D >) -> Self { Self :: from (Matrix :: from_superset_unchecked (& m . coords)) } }
};
}
