// Generated macro for impl_1904 (impl)
macro_rules! Depcrate_geometry_unit_compleximpl_1904 {
() => {
// Module: crate::geometry::unit_complex
// Provides: {"impl_1904"}
// Dependencies: {}
impl < T : RealField > RelativeEq for UnitComplex < T > { # [inline] fn default_max_relative () -> Self :: Epsilon { T :: default_max_relative () } # [inline] fn relative_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { self . re . relative_eq (& other . re , epsilon . clone () , max_relative . clone ()) && self . im . relative_eq (& other . im , epsilon , max_relative) } }
};
}
