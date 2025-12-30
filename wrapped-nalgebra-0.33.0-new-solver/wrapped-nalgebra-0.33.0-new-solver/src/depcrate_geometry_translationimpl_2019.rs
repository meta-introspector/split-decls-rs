// Generated macro for impl_2019 (impl)
macro_rules! Depcrate_geometry_translationimpl_2019 {
() => {
// Module: crate::geometry::translation
// Provides: {"impl_2019"}
// Dependencies: {}
impl < T : Scalar + RelativeEq , const D : usize > RelativeEq for Translation < T , D > where T :: Epsilon : Clone , { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . vector . relative_eq (& other . vector , epsilon , max_relative) } }
};
}
