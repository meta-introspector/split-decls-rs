// Generated macro for impl_2290 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2290 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2290"}
// Dependencies: {}
impl < T1 , T2 , R , C , const D : usize > SubsetOf < Transform < T2 , C , D > > for Isometry < T1 , R , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , C : SuperTCategoryOf < TAffine > , R : AbstractRotation < T1 , D > + SubsetOf < OMatrix < T1 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > > + SubsetOf < OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > > , Const < D > : DimNameAdd < U1 > + DimMin < Const < D > , Output = Const < D > > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn to_superset (& self) -> Transform < T2 , C , D > { Transform :: from_matrix_unchecked (self . to_homogeneous () . to_superset ()) } # [inline] fn is_in_subset (t : & Transform < T2 , C , D >) -> bool { < Self as SubsetOf < _ > > :: is_in_subset (t . matrix ()) } # [inline] fn from_superset_unchecked (t : & Transform < T2 , C , D >) -> Self { Self :: from_superset_unchecked (t . matrix ()) } }
};
}
