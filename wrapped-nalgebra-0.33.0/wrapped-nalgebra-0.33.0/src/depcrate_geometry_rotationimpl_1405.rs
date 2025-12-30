// Generated macro for impl_1405 (impl)
macro_rules! Depcrate_geometry_rotationimpl_1405 {
() => {
// Module: crate::geometry::rotation
// Provides: {"impl_1405"}
// Dependencies: {}
impl < T , const D : usize > AbsDiffEq for Rotation < T , D > where T : Scalar + AbsDiffEq , T :: Epsilon : Clone , { type Epsilon = T :: Epsilon ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . matrix . abs_diff_eq (& other . matrix , epsilon) } }
};
}
