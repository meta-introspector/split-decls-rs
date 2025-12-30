// Generated macro for impl_2419 (impl)
macro_rules! Depcrate_geometry_similarity_conversionimpl_2419 {
() => {
// Module: crate::geometry::similarity_conversion
// Provides: {"impl_2419"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > From < Similarity < T , R , D > > for OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > where Const < D > : DimNameAdd < U1 > , R : SubsetOf < OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn from (sim : Similarity < T , R , D >) -> Self { sim . to_homogeneous () } }
};
}
