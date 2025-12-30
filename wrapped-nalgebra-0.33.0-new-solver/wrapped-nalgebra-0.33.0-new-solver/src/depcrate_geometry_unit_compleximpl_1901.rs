// Generated macro for impl_1901 (impl)
macro_rules! Depcrate_geometry_unit_compleximpl_1901 {
() => {
// Module: crate::geometry::unit_complex
// Provides: {"impl_1901"}
// Dependencies: {}
# [doc = " # Interpolation"] impl < T : SimdRealField > UnitComplex < T > where T :: Element : SimdRealField , { # [doc = " Spherical linear interpolation between two rotations represented as unit complex numbers."] # [doc = ""] # [doc = " # Examples:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use nalgebra::geometry::UnitComplex;"] # [doc = ""] # [doc = " let rot1 = UnitComplex::new(std::f32::consts::FRAC_PI_4);"] # [doc = " let rot2 = UnitComplex::new(-std::f32::consts::PI);"] # [doc = ""] # [doc = " let rot = rot1.slerp(&rot2, 1.0 / 3.0);"] # [doc = ""] # [doc = " assert_relative_eq!(rot.angle(), std::f32::consts::FRAC_PI_2);"] # [doc = " ```"] # [inline] # [must_use] pub fn slerp (& self , other : & Self , t : T) -> Self { let delta = other / self ; self * Self :: new (delta . angle () * t) } }
};
}
