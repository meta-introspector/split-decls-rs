// Generated macro for impl_2551 (impl)
macro_rules! Depcrate_geometry_transform_conversionimpl_2551 {
() => {
// Module: crate::geometry::transform_conversion
// Provides: {"impl_2551"}
// Dependencies: {}
impl < T1 , T2 , C , const D : usize > SubsetOf < OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > > for Transform < T1 , C , D > where T1 : RealField + SubsetOf < T2 > , T2 : RealField , C : TCategory , Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , T1 :: Epsilon : Copy , T2 :: Epsilon : Copy , { # [inline] fn to_superset (& self) -> OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > { self . matrix () . to_superset () } # [inline] fn is_in_subset (m : & OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > >) -> bool { C :: check_homogeneous_invariants (m) } # [inline] fn from_superset_unchecked (m : & OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > ,) -> Self { Self :: from_matrix_unchecked (crate :: convert_ref_unchecked (m)) } }
};
}
