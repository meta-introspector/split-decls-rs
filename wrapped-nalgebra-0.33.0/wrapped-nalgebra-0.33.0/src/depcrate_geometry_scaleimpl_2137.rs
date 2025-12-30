// Generated macro for impl_2137 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2137 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2137"}
// Dependencies: {}
impl < T : Scalar + RelativeEq , const D : usize > RelativeEq for Scale < T , D > where T :: Epsilon : Clone , { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . vector . relative_eq (& other . vector , epsilon , max_relative) } }
};
}
