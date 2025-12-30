// Generated macro for impl_2523 (impl)
macro_rules! Depcrate_geometry_transformimpl_2523 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2523"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > RelativeEq for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , T :: Epsilon : Clone , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . matrix . relative_eq (& other . matrix , epsilon , max_relative) } }
};
}
