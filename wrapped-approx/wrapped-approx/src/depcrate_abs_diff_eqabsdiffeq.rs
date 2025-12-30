// Generated macro for AbsDiffEq (trait)
macro_rules! Depcrate_abs_diff_eqAbsDiffEq {
() => {
// Module: crate::abs_diff_eq
// Provides: {"AbsDiffEq"}
// Dependencies: {}
# [doc = " Equality that is defined using the absolute difference of two numbers."] pub trait AbsDiffEq < Rhs = Self > : PartialEq < Rhs > where Rhs : ? Sized , { # [doc = " Used for specifying relative comparisons."] type Epsilon ; # [doc = " The default tolerance to use when testing values that are close together."] # [doc = ""] # [doc = " This is used when no `epsilon` value is supplied to the [`abs_diff_eq!`], [`relative_eq!`],"] # [doc = " or [`ulps_eq!`] macros."] fn default_epsilon () -> Self :: Epsilon ; # [doc = " A test for equality that uses the absolute difference to compute the approximate"] # [doc = " equality of two numbers."] fn abs_diff_eq (& self , other : & Rhs , epsilon : Self :: Epsilon) -> bool ; # [doc = " The inverse of [`AbsDiffEq::abs_diff_eq`]."] fn abs_diff_ne (& self , other : & Rhs , epsilon : Self :: Epsilon) -> bool { ! Self :: abs_diff_eq (self , other , epsilon) } }
};
}
