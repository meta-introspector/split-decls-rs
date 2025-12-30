// Generated macro for impl_2517 (impl)
macro_rules! Depcrate_geometry_transformimpl_2517 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2517"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > PartialEq for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn eq (& self , right : & Self) -> bool { self . matrix == right . matrix } }
};
}
