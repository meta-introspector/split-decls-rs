// Generated macro for impl_2511 (impl)
macro_rules! Depcrate_geometry_transformimpl_2511 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2511"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > Clone for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn clone (& self) -> Self { Transform :: from_matrix_unchecked (self . matrix . clone ()) } }
};
}
