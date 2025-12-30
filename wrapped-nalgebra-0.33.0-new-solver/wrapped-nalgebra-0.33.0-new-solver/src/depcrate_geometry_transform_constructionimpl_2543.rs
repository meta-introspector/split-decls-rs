// Generated macro for impl_2543 (impl)
macro_rules! Depcrate_geometry_transform_constructionimpl_2543 {
() => {
// Module: crate::geometry::transform_construction
// Provides: {"impl_2543"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > One for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [doc = " Creates a new identity transform."] # [inline] fn one () -> Self { Self :: identity () } }
};
}
