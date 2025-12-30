// Generated macro for impl_1273 (impl)
macro_rules! Depcrate_geometry_pointimpl_1273 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1273"}
// Dependencies: {}
impl < T : Scalar + UlpsEq , D : DimName > UlpsEq for OPoint < T , D > where T :: Epsilon : Clone , DefaultAllocator : Allocator < D > , { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . coords . ulps_eq (& other . coords , epsilon , max_ulps) } }
};
}
