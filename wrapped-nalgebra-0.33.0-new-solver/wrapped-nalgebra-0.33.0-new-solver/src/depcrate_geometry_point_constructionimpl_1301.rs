// Generated macro for impl_1301 (impl)
macro_rules! Depcrate_geometry_point_constructionimpl_1301 {
() => {
// Module: crate::geometry::point_construction
// Provides: {"impl_1301"}
// Dependencies: {}
impl < T : Scalar + Bounded , D : DimName > Bounded for OPoint < T , D > where DefaultAllocator : Allocator < D > , { # [inline] fn max_value () -> Self { Self :: from (OVector :: max_value ()) } # [inline] fn min_value () -> Self { Self :: from (OVector :: min_value ()) } }
};
}
