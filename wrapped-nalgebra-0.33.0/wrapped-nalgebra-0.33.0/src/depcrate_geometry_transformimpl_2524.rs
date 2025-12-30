// Generated macro for impl_2524 (impl)
macro_rules! Depcrate_geometry_transformimpl_2524 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2524"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > UlpsEq for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , T :: Epsilon : Clone , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . matrix . ulps_eq (& other . matrix , epsilon , max_ulps) } }
};
}
