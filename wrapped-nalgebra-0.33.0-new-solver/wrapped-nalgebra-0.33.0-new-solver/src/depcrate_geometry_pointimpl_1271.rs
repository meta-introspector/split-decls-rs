// Generated macro for impl_1271 (impl)
macro_rules! Depcrate_geometry_pointimpl_1271 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1271"}
// Dependencies: {}
impl < T : Scalar + AbsDiffEq , D : DimName > AbsDiffEq for OPoint < T , D > where T :: Epsilon : Clone , DefaultAllocator : Allocator < D > , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . coords . abs_diff_eq (& other . coords , epsilon) } }
};
}
