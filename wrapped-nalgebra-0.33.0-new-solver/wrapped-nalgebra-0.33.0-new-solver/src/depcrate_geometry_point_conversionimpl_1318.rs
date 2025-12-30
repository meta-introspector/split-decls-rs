// Generated macro for impl_1318 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1318 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1318"}
// Dependencies: {}
impl < T : Scalar + Zero + One , D : DimName > From < OPoint < T , D > > for OVector < T , DimNameSum < D , U1 > > where D : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < D , U1 > > + Allocator < D > , { # [inline] fn from (t : OPoint < T , D >) -> Self { t . to_homogeneous () } }
};
}
