// Generated macro for impl_1903 (impl)
macro_rules! Depcrate_geometry_unit_compleximpl_1903 {
() => {
// Module: crate::geometry::unit_complex
// Provides: {"impl_1903"}
// Dependencies: {}
impl < T : RealField > AbsDiffEq for UnitComplex < T > { type Epsilon = T ; # [inline] fn default_epsilon () -> Self :: Epsilon { T :: default_epsilon () } # [inline] fn abs_diff_eq (& self , other : & Self , epsilon : Self :: Epsilon) -> bool { self . re . abs_diff_eq (& other . re , epsilon . clone ()) && self . im . abs_diff_eq (& other . im , epsilon) } }
};
}
