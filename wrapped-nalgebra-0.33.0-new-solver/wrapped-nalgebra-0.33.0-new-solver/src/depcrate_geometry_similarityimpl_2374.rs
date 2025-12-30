// Generated macro for impl_2374 (impl)
macro_rules! Depcrate_geometry_similarityimpl_2374 {
() => {
// Module: crate::geometry::similarity
// Provides: {"impl_2374"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > Similarity < T , R , D > { # [doc = " Converts this similarity into its equivalent homogeneous transformation matrix."] # [inline] # [must_use] pub fn to_homogeneous (& self) -> OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > where Const < D > : DimNameAdd < U1 > , R : SubsetOf < OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { let mut res = self . isometry . to_homogeneous () ; for e in res . fixed_view_mut :: < D , D > (0 , 0) . iter_mut () { * e *= self . scaling . clone () } res } }
};
}
