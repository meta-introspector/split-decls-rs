// Generated macro for impl_2136 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2136 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2136"}
// Dependencies: {}
impl < T : Scalar + AbsDiffEq , const D : usize > AbsDiffEq for Scale < T , D > where T :: Epsilon : Clone , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . vector . abs_diff_eq (& other . vector , epsilon) } }
};
}
