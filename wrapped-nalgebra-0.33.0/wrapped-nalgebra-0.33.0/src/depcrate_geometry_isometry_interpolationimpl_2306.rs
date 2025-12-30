// Generated macro for impl_2306 (impl)
macro_rules! Depcrate_geometry_isometry_interpolationimpl_2306 {
() => {
// Module: crate::geometry::isometry_interpolation
// Provides: {"impl_2306"}
// Dependencies: {}
impl < T : SimdRealField > IsometryMatrix2 < T > { # [doc = " Interpolates between two isometries using a linear interpolation for the translation part,"] # [doc = " and a spherical interpolation for the rotation part."] # [doc = ""] # [doc = " Panics if the angle between both rotations is 180 degrees (in which case the interpolation"] # [doc = " is not well-defined). Use `.try_lerp_slerp` instead to avoid the panic."] # [doc = ""] # [doc = " # Examples:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use nalgebra::{Vector2, Translation2, Rotation2, IsometryMatrix2};"] # [doc = ""] # [doc = " let t1 = Translation2::new(1.0, 2.0);"] # [doc = " let t2 = Translation2::new(4.0, 8.0);"] # [doc = " let q1 = Rotation2::new(std::f32::consts::FRAC_PI_4);"] # [doc = " let q2 = Rotation2::new(-std::f32::consts::PI);"] # [doc = " let iso1 = IsometryMatrix2::from_parts(t1, q1);"] # [doc = " let iso2 = IsometryMatrix2::from_parts(t2, q2);"] # [doc = ""] # [doc = " let iso3 = iso1.lerp_slerp(&iso2, 1.0 / 3.0);"] # [doc = ""] # [doc = " assert_eq!(iso3.translation.vector, Vector2::new(2.0, 4.0));"] # [doc = " assert_relative_eq!(iso3.rotation.angle(), std::f32::consts::FRAC_PI_2);"] # [doc = " ```"] # [inline] # [must_use] pub fn lerp_slerp (& self , other : & Self , t : T) -> Self where T : RealField , { let tr = self . translation . vector . lerp (& other . translation . vector , t . clone ()) ; let rot = self . rotation . slerp (& other . rotation , t) ; Self :: from_parts (tr . into () , rot) } }
};
}
