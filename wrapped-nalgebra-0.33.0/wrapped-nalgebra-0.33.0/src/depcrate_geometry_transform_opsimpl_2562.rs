// Generated macro for impl_2562 (impl)
macro_rules! Depcrate_geometry_transform_opsimpl_2562 {
() => {
// Module: crate::geometry::transform_ops
// Provides: {"impl_2562"}
// Dependencies: {}
impl < T : RealField , const D : usize > IndexMut < (usize , usize) > for Transform < T , TGeneral , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn index_mut (& mut self , ij : (usize , usize)) -> & mut T { self . matrix_mut () . index_mut (ij) } }
};
}
