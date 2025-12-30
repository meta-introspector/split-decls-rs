// Generated macro for impl_2508 (impl)
macro_rules! Depcrate_geometry_transformimpl_2508 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2508"}
// Dependencies: {}
impl < T : RealField + Debug , C : TCategory , const D : usize > Debug for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . matrix . fmt (formatter) } }
};
}
