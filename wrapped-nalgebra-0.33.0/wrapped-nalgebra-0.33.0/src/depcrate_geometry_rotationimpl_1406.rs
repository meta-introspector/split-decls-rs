// Generated macro for impl_1406 (impl)
macro_rules! Depcrate_geometry_rotationimpl_1406 {
() => {
// Module: crate::geometry::rotation
// Provides: {"impl_1406"}
// Dependencies: {}
impl < T , const D : usize > RelativeEq for Rotation < T , D > where T : Scalar + RelativeEq , T :: Epsilon : Clone , { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . matrix . relative_eq (& other . matrix , epsilon , max_relative) } }
};
}
