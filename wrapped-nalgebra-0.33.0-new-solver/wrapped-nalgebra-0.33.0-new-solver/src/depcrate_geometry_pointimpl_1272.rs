// Generated macro for impl_1272 (impl)
macro_rules! Depcrate_geometry_pointimpl_1272 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1272"}
// Dependencies: {}
impl < T : Scalar + RelativeEq , D : DimName > RelativeEq for OPoint < T , D > where T :: Epsilon : Clone , DefaultAllocator : Allocator < D > , { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . coords . relative_eq (& other . coords , epsilon , max_relative) } }
};
}
