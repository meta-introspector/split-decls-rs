// Generated macro for impl_2173 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2173 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2173"}
// Dependencies: {}
impl < T1 , T2 , const D : usize > SubsetOf < OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > > for Scale < T1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > + Allocator < DimNameSum < Const < D > , U1 > , U1 > , { # [inline] fn to_superset (& self) -> OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > { self . to_homogeneous () . to_superset () } # [inline] fn is_in_subset (m : & OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > >) -> bool { if m [(D , D)] != T2 :: one () { return false ; } for i in 0 .. D + 1 { for j in 0 .. D + 1 { if i != j && m [(i , j)] != T2 :: zero () { return false ; } } } true } # [inline] fn from_superset_unchecked (m : & OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > ,) -> Self { let v = m . fixed_view :: < D , D > (0 , 0) . diagonal () ; Self { vector : crate :: convert_unchecked (v) , } } }
};
}
