// Generated macro for impl_2513 (impl)
macro_rules! Depcrate_geometry_transformimpl_2513 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2513"}
// Dependencies: {}
# [cfg (feature = "bytemuck")] unsafe impl < T , C : TCategory , const D : usize > bytemuck :: Pod for Transform < T , C , D > where T : RealField + bytemuck :: Pod , Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > : bytemuck :: Pod , Owned < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > : Copy , { }
};
}
