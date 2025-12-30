// Generated macro for impl_2509 (impl)
macro_rules! Depcrate_geometry_transformimpl_2509 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2509"}
// Dependencies: {}
impl < T : RealField + hash :: Hash , C : TCategory , const D : usize > hash :: Hash for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , Owned < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > : hash :: Hash , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . matrix . hash (state) ; } }
};
}
