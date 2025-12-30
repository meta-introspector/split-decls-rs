// Generated macro for AbsDiff (struct)
macro_rules! DepcrateAbsDiff {
() => {
// Module: crate
// Provides: {"AbsDiff"}
// Dependencies: {}
# [doc = " The requisite parameters for testing for approximate equality using a"] # [doc = " absolute difference based comparison."] # [doc = ""] # [doc = " This is not normally used directly, rather via the"] # [doc = " `assert_abs_diff_{eq|ne}!` and `abs_diff_{eq|ne}!` macros."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::f64;"] # [doc = " use approx::AbsDiff;"] # [doc = ""] # [doc = " AbsDiff::default().eq(&1.0, &1.0);"] # [doc = " AbsDiff::default().epsilon(f64::EPSILON).eq(&1.0, &1.0);"] # [doc = " ```"] pub struct AbsDiff < A , B = A > where A : AbsDiffEq < B > + ? Sized , B : ? Sized , { # [doc = " The tolerance to use when testing values that are close together."] pub epsilon : A :: Epsilon , }
};
}
