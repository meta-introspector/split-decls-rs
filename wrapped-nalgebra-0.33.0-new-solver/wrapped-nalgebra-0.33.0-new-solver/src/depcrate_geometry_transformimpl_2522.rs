// Generated macro for impl_2522 (impl)
macro_rules! Depcrate_geometry_transformimpl_2522 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2522"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > AbsDiffEq for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , T :: Epsilon : Clone , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . matrix . abs_diff_eq (& other . matrix , epsilon) } }
};
}
