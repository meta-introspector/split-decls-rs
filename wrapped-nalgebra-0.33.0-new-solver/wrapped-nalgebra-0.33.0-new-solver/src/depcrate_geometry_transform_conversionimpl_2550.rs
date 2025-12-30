// Generated macro for impl_2550 (impl)
macro_rules! Depcrate_geometry_transform_conversionimpl_2550 {
() => {
// Module: crate::geometry::transform_conversion
// Provides: {"impl_2550"}
// Dependencies: {}
impl < T1 , T2 , C1 , C2 , const D : usize > SubsetOf < Transform < T2 , C2 , D > > for Transform < T1 , C1 , D > where T1 : RealField + SubsetOf < T2 > , T2 : RealField , C1 : TCategory , C2 : SuperTCategoryOf < C1 > , Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , T1 :: Epsilon : Copy , T2 :: Epsilon : Copy , { # [inline] fn to_superset (& self) -> Transform < T2 , C2 , D > { Transform :: from_matrix_unchecked (self . to_homogeneous () . to_superset ()) } # [inline] fn is_in_subset (t : & Transform < T2 , C2 , D >) -> bool { < Self as SubsetOf < _ > > :: is_in_subset (t . matrix ()) } # [inline] fn from_superset_unchecked (t : & Transform < T2 , C2 , D >) -> Self { Self :: from_superset_unchecked (t . matrix ()) } }
};
}
