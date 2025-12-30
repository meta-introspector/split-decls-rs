// Generated macro for Relative (struct)
macro_rules! DepcrateRelative {
() => {
// Module: crate
// Provides: {"Relative"}
// Dependencies: {}
# [doc = " The requisite parameters for testing for approximate equality using a"] # [doc = " relative based comparison."] # [doc = ""] # [doc = " This is not normally used directly, rather via the"] # [doc = " `assert_relative_{eq|ne}!` and `relative_{eq|ne}!` macros."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::f64;"] # [doc = " use approx::Relative;"] # [doc = ""] # [doc = " Relative::default().eq(&1.0, &1.0);"] # [doc = " Relative::default().epsilon(f64::EPSILON).eq(&1.0, &1.0);"] # [doc = " Relative::default().max_relative(1.0).eq(&1.0, &1.0);"] # [doc = " Relative::default().epsilon(f64::EPSILON).max_relative(1.0).eq(&1.0, &1.0);"] # [doc = " Relative::default().max_relative(1.0).epsilon(f64::EPSILON).eq(&1.0, &1.0);"] # [doc = " ```"] pub struct Relative < A , B = A > where A : RelativeEq < B > + ? Sized , B : ? Sized , { # [doc = " The tolerance to use when testing values that are close together."] pub epsilon : A :: Epsilon , # [doc = " The relative tolerance for testing values that are far-apart."] pub max_relative : A :: Epsilon , }
};
}
