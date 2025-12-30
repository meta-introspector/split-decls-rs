// Generated macro for impl_2541 (impl)
macro_rules! Depcrate_geometry_transform_constructionimpl_2541 {
() => {
// Module: crate::geometry::transform_construction
// Provides: {"impl_2541"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > Default for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { fn default () -> Self { Self :: identity () } }
};
}
