// Generated macro for impl_2293 (impl)
macro_rules! Depcrate_geometry_isometry_conversionimpl_2293 {
() => {
// Module: crate::geometry::isometry_conversion
// Provides: {"impl_2293"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > From < Isometry < T , R , D > > for OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > where Const < D > : DimNameAdd < U1 > , R : SubsetOf < OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn from (iso : Isometry < T , R , D >) -> Self { iso . to_homogeneous () } }
};
}
