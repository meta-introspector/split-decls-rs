// Generated macro for Gcd (trait)
macro_rules! Depcrate_traitsGcd {
() => {
// Module: crate::traits
// Provides: {"Gcd"}
// Dependencies: {}
# [doc = " Compute the greatest common divisor of two integers."] pub trait Gcd < Rhs = Self > : Sized { # [doc = " Output type."] type Output ; # [doc = " Compute the greatest common divisor of `self` and `rhs`."] fn gcd (& self , rhs : & Rhs) -> Self :: Output ; # [doc = " Compute the greatest common divisor of `self` and `rhs` in variable time."] fn gcd_vartime (& self , rhs : & Rhs) -> Self :: Output ; }
};
}
