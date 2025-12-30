// Generated macro for impl_1449 (impl)
macro_rules! Depcrate_geometry_rotation_interpolationimpl_1449 {
() => {
// Module: crate::geometry::rotation_interpolation
// Provides: {"impl_1449"}
// Dependencies: {}
# [doc = " # Interpolation"] impl < T : SimdRealField > Rotation2 < T > { # [doc = " Spherical linear interpolation between two rotation matrices."] # [doc = ""] # [doc = " # Examples:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use nalgebra::geometry::Rotation2;"] # [doc = ""] # [doc = " let rot1 = Rotation2::new(std::f32::consts::FRAC_PI_4);"] # [doc = " let rot2 = Rotation2::new(-std::f32::consts::PI);"] # [doc = ""] # [doc = " let rot = rot1.slerp(&rot2, 1.0 / 3.0);"] # [doc = ""] # [doc = " assert_relative_eq!(rot.angle(), std::f32::consts::FRAC_PI_2);"] # [doc = " ```"] # [inline] # [must_use] pub fn slerp (& self , other : & Self , t : T) -> Self where T :: Element : SimdRealField , { let c1 = UnitComplex :: from (self . clone ()) ; let c2 = UnitComplex :: from (other . clone ()) ; c1 . slerp (& c2 , t) . into () } }
};
}
