// Generated macro for impl_1317 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1317 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1317"}
// Dependencies: {}
impl < T1 , T2 , D > SubsetOf < OVector < T2 , DimNameSum < D , U1 > > > for OPoint < T1 , D > where D : DimNameAdd < U1 > , T1 : Scalar , T2 : Scalar + Zero + One + ClosedDivAssign + SupersetOf < T1 > , DefaultAllocator : Allocator < D > + Allocator < DimNameSum < D , U1 > > , { # [inline] fn to_superset (& self) -> OVector < T2 , DimNameSum < D , U1 > > { let p : OPoint < T2 , D > = self . to_superset () ; p . to_homogeneous () } # [inline] fn is_in_subset (v : & OVector < T2 , DimNameSum < D , U1 > >) -> bool { crate :: is_convertible :: < _ , OVector < T1 , DimNameSum < D , U1 > > > (v) && ! v [D :: dim ()] . is_zero () } # [inline] fn from_superset_unchecked (v : & OVector < T2 , DimNameSum < D , U1 > >) -> Self { let coords = v . generic_view ((0 , 0) , (D :: name () , Const :: < 1 >)) / v [D :: dim ()] . clone () ; Self { coords : crate :: convert_unchecked (coords) , } } }
};
}
