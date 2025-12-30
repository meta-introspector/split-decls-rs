// Generated macro for impl_2552 (impl)
macro_rules! Depcrate_geometry_transform_conversionimpl_2552 {
() => {
// Module: crate::geometry::transform_conversion
// Provides: {"impl_2552"}
// Dependencies: {}
impl < T : RealField , C , const D : usize > From < Transform < T , C , D > > for OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > where Const < D > : DimNameAdd < U1 > , C : TCategory , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn from (t : Transform < T , C , D >) -> Self { t . to_homogeneous () } }
};
}
