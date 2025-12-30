// Generated macro for impl_1492 (impl)
macro_rules! Depcrate_geometry_rotation_specializationimpl_1492 {
() => {
// Module: crate::geometry::rotation_specialization
// Provides: {"impl_1492"}
// Dependencies: {}
# [doc = " # 2D angle extraction"] impl < T : SimdRealField > Rotation2 < T > { # [doc = " The rotation angle."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use nalgebra::Rotation2;"] # [doc = " let rot = Rotation2::new(1.78);"] # [doc = " assert_relative_eq!(rot.angle(), 1.78);"] # [doc = " ```"] # [inline] # [must_use] pub fn angle (& self) -> T { self . matrix () [(1 , 0)] . clone () . simd_atan2 (self . matrix () [(0 , 0)] . clone ()) } # [doc = " The rotation angle needed to make `self` and `other` coincide."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use nalgebra::Rotation2;"] # [doc = " let rot1 = Rotation2::new(0.1);"] # [doc = " let rot2 = Rotation2::new(1.7);"] # [doc = " assert_relative_eq!(rot1.angle_to(&rot2), 1.6);"] # [doc = " ```"] # [inline] # [must_use] pub fn angle_to (& self , other : & Self) -> T { self . rotation_to (other) . angle () } # [doc = " The rotation angle returned as a 1-dimensional vector."] # [doc = ""] # [doc = " This is generally used in the context of generic programming. Using"] # [doc = " the `.angle()` method instead is more common."] # [inline] # [must_use] pub fn scaled_axis (& self) -> SVector < T , 1 > { Vector1 :: new (self . angle ()) } }
};
}
