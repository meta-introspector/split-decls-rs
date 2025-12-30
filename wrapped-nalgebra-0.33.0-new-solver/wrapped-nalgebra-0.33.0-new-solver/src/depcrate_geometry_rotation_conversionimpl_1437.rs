// Generated macro for impl_1437 (impl)
macro_rules! Depcrate_geometry_rotation_conversionimpl_1437 {
() => {
// Module: crate::geometry::rotation_conversion
// Provides: {"impl_1437"}
// Dependencies: {}
impl < T1 , T2 , C , const D : usize > SubsetOf < Transform < T2 , C , D > > for Rotation < T1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , C : SuperTCategoryOf < TAffine > , Const < D > : DimNameAdd < U1 > + DimMin < Const < D > , Output = Const < D > > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn to_superset (& self) -> Transform < T2 , C , D > { Transform :: from_matrix_unchecked (self . to_homogeneous () . to_superset ()) } # [inline] fn is_in_subset (t : & Transform < T2 , C , D >) -> bool { < Self as SubsetOf < _ > > :: is_in_subset (t . matrix ()) } # [inline] fn from_superset_unchecked (t : & Transform < T2 , C , D >) -> Self { Self :: from_superset_unchecked (t . matrix ()) } }
};
}
