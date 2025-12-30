// Generated macro for RelativeEq (trait)
macro_rules! Depcrate_relative_eqRelativeEq {
() => {
// Module: crate::relative_eq
// Provides: {"RelativeEq"}
// Dependencies: {}
# [doc = " Equality comparisons between two numbers using both the absolute difference and"] # [doc = " relative based comparisons."] pub trait RelativeEq < Rhs = Self > : AbsDiffEq < Rhs > where Rhs : ? Sized , { # [doc = " The default relative tolerance for testing values that are far-apart."] # [doc = ""] # [doc = " This is used when no `max_relative` value is supplied to the [`relative_eq`] macro."] fn default_max_relative () -> Self :: Epsilon ; # [doc = " A test for equality that uses a relative comparison if the values are far apart."] fn relative_eq (& self , other : & Rhs , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon) -> bool ; # [doc = " The inverse of [`RelativeEq::relative_eq`]."] fn relative_ne (& self , other : & Rhs , epsilon : Self :: Epsilon , max_relative : Self :: Epsilon ,) -> bool { ! Self :: relative_eq (self , other , epsilon , max_relative) } }
};
}
