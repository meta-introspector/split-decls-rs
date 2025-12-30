// Generated macro for impl_1303 (impl)
macro_rules! Depcrate_geometry_point_constructionimpl_1303 {
() => {
// Module: crate::geometry::point_construction
// Provides: {"impl_1303"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : Scalar + Arbitrary + Send , D : DimName > Arbitrary for OPoint < T , D > where < DefaultAllocator as Allocator < D > > :: Buffer < T > : Send , DefaultAllocator : Allocator < D > , { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: from (OVector :: arbitrary (g)) } }
};
}
