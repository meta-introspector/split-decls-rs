// Generated macro for impl_2561 (impl)
macro_rules! Depcrate_geometry_transform_opsimpl_2561 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"impl_2561"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > Index < (usize , usize) > for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { type Output = T ; # [inline] fn index (& self , ij : (usize , usize)) -> & T { self . matrix () . index (ij) } }
};
}
