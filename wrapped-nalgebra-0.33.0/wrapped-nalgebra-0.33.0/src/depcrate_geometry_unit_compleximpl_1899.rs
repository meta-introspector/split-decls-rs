// Generated macro for impl_1899 (impl)
macro_rules! Depcrate_geometry_unit_compleximpl_1899 {
() => {
// Module: crate::geometry::unit_complex
// Provides: {"impl_1899"}
// Dependencies: {}
# [doc = " # Conversion to a matrix"] impl < T : SimdRealField > UnitComplex < T > where T :: Element : SimdRealField , { # [doc = " Builds the rotation matrix corresponding to this unit complex number."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{UnitComplex, Rotation2};"] # [doc = " # use std::f32;"] # [doc = " let rot = UnitComplex::new(f32::consts::FRAC_PI_6);"] # [doc = " let expected = Rotation2::new(f32::consts::FRAC_PI_6);"] # [doc = " assert_eq!(rot.to_rotation_matrix(), expected);"] # [doc = " ```"] # [inline] # [must_use] pub fn to_rotation_matrix (self) -> Rotation2 < T > { let r = self . re . clone () ; let i = self . im . clone () ; Rotation2 :: from_matrix_unchecked (Matrix2 :: new (r . clone () , - i . clone () , i , r)) } # [doc = " Converts this unit complex number into its equivalent homogeneous transformation matrix."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{UnitComplex, Matrix3};"] # [doc = " # use std::f32;"] # [doc = " let rot = UnitComplex::new(f32::consts::FRAC_PI_6);"] # [doc = " let expected = Matrix3::new(0.8660254, -0.5,      0.0,"] # [doc = "                             0.5,       0.8660254, 0.0,"] # [doc = "                             0.0,       0.0,       1.0);"] # [doc = " assert_eq!(rot.to_homogeneous(), expected);"] # [doc = " ```"] # [inline] # [must_use] pub fn to_homogeneous (self) -> Matrix3 < T > { self . to_rotation_matrix () . to_homogeneous () } }
};
}
