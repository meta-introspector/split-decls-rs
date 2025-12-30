// Generated macro for impl_2059 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2059 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2059"}
// Dependencies: {}
impl < T1 , T2 , const D : usize > SubsetOf < OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > > for Translation < T1 , D > where T1 : RealField , T2 : RealField + SupersetOf < T1 > , Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < Const < D > , Buffer < T1 > = ArrayStorage < T1 , D , 1 > > + Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn to_superset (& self) -> OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > { self . to_homogeneous () . to_superset () } # [inline] fn is_in_subset (m : & OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > >) -> bool { let id = m . generic_view ((0 , 0) , (DimNameSum :: < Const < D > , U1 > :: name () , Const :: < D >)) ; m . iter () . all (| e | SupersetOf :: < T1 > :: is_in_subset (e)) && id . is_identity (T2 :: zero ()) && m [(D , D)] == T2 :: one () } # [inline] fn from_superset_unchecked (m : & OMatrix < T2 , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > ,) -> Self { let t : OVector < T2 , Const < D > > = m . fixed_view :: < D , 1 > (0 , D) . into_owned () ; Self { vector : crate :: convert_unchecked (t) , } } }
};
}
