// Generated macro for impl_2512 (impl)
macro_rules! Depcrate_geometry_transformimpl_2512 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2512"}
// Dependencies: {}
# [cfg (feature = "bytemuck")] unsafe impl < T , C : TCategory , const D : usize > bytemuck :: Zeroable for Transform < T , C , D > where T : RealField + bytemuck :: Zeroable , Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > : bytemuck :: Zeroable , { }
};
}
