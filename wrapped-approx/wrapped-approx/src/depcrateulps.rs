// Generated macro for Ulps (struct)
macro_rules! DepcrateUlps {
() => {
// Module: crate
// Provides: {"Ulps"}
// Dependencies: {}
# [doc = " The requisite parameters for testing for approximate equality using an ULPs"] # [doc = " based comparison."] # [doc = ""] # [doc = " This is not normally used directly, rather via the `assert_ulps_{eq|ne}!`"] # [doc = " and `ulps_{eq|ne}!` macros."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::f64;"] # [doc = " use approx::Ulps;"] # [doc = ""] # [doc = " Ulps::default().eq(&1.0, &1.0);"] # [doc = " Ulps::default().epsilon(f64::EPSILON).eq(&1.0, &1.0);"] # [doc = " Ulps::default().max_ulps(4).eq(&1.0, &1.0);"] # [doc = " Ulps::default().epsilon(f64::EPSILON).max_ulps(4).eq(&1.0, &1.0);"] # [doc = " Ulps::default().max_ulps(4).epsilon(f64::EPSILON).eq(&1.0, &1.0);"] # [doc = " ```"] pub struct Ulps < A , B = A > where A : UlpsEq < B > + ? Sized , B : ? Sized , { # [doc = " The tolerance to use when testing values that are close together."] pub epsilon : A :: Epsilon , # [doc = " The ULPs to tolerate when testing values that are far-apart."] pub max_ulps : u32 , }
};
}
