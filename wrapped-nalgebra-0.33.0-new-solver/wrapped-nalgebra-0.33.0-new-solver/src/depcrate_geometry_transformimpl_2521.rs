// Generated macro for impl_2521 (impl)
macro_rules! Depcrate_geometry_transformimpl_2521 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2521"}
// Dependencies: {}
impl < T : RealField , const D : usize > Transform < T , TGeneral , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [doc = " A mutable reference to underlying matrix. Use `.matrix_mut_unchecked` instead if this"] # [doc = " transformation category is not `TGeneral`."] # [inline] pub fn matrix_mut (& mut self ,) -> & mut OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > { self . matrix_mut_unchecked () } }
};
}
